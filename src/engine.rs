use crate::{
    slice_ext::ByteSliceExt as _,
    types::{Channel, Instrument, ModSrc, PlaybackState},
};

/// A micromod decoding instance thingy.
pub struct Engine<'src> {
    pub(crate) sample_rate: i32,
    pub(crate) channels: Vec<Channel>,
    pub(crate) src: ModSrc<'src>,
    pub(crate) playback: PlaybackState,
}

impl Engine<'_> {
    /// Create a new micromod decoder apparatus.
    pub fn new(data: &[u8], sample_rate: i32) -> Result<Engine, InitError> {
        let num_channels = match crate::parse::calculate_num_channels(data) {
            Some(num_channels) => num_channels,
            None => return Err(InitError::ChannelNumIncorrect),
        };
        if sample_rate < 8000 {
            return Err(InitError::SamplingRateIncorrect);
        }
        let song_length = i32::from(data[950]) & 0x7f;
        let sequence = &data[952..];
        let pattern_data = &data[1084..];
        let num_patterns = crate::parse::calculate_num_patterns(data);
        let mut sample_data_offset: usize =
            1084 + num_patterns as usize * 64 * num_channels as usize * 4;
        let instruments = std::array::from_fn(|inst_idx| {
            if inst_idx == 0 {
                // First instrument is an unused dummy instrument
                Instrument::dummy()
            } else {
                let sample_length = u32::from(data.read_u16_be(inst_idx * 30 + 12).unwrap()) * 2;
                let fine_tune = i32::from(data[inst_idx * 30 + 14]) & 0xf;
                let fine_tune = ((fine_tune & 0x7) - (fine_tune & 0x8) + 8) as u8;
                let volume = i32::from(data[inst_idx * 30 + 15]) & 0x7f;
                let volume = (if volume > 64 { 64 } else { volume }) as u8;

                let mut loop_start = u32::from(data.read_u16_be(inst_idx * 30 + 16).unwrap()) * 2;
                let mut loop_length = u32::from(data.read_u16_be(inst_idx * 30 + 18).unwrap()) * 2;

                if loop_start + loop_length > sample_length {
                    if loop_start / 2 + loop_length <= sample_length {
                        loop_start /= 2;
                    } else {
                        loop_length = sample_length - loop_start;
                    }
                }
                if loop_length < 4 {
                    loop_start = sample_length;
                    loop_length = 0;
                }
                let loop_start = loop_start << 14;
                let loop_length = loop_length << 14;
                let sample_data = bytemuck::cast_slice(&data[sample_data_offset..]);
                sample_data_offset += sample_length as usize;
                Instrument {
                    volume,
                    fine_tune,
                    loop_start: loop_start as usize,
                    loop_length: loop_length as usize,
                    sample_data,
                }
            }
        });
        let mut mm = Engine {
            sample_rate,
            channels: vec![Channel::default(); num_channels as usize],
            src: ModSrc {
                instruments,
                pattern_data,
                sequence,
                num_channels: num_channels.into(),
                song_length,
            },
            playback: PlaybackState::default(),
        };
        mm.playback.c2_rate = if mm.src.num_channels > 4 { 8363 } else { 8287 };
        mm.playback.gain = if mm.src.num_channels > 4 { 32 } else { 64 };
        mm.unmute_all();
        mm.set_position(0);
        Ok(mm)
    }
    /// Fill a buffer with delicious samples
    pub fn get_audio(&mut self, output_buffer: &mut [i16], mut count: usize) -> bool {
        if self.channels.is_empty() {
            return false;
        }
        let mut offset = 0;
        let mut cnt = true;
        while count > 0 {
            let mut remain = self.playback.tick_len - self.playback.tick_offset;
            if remain > count as i32 {
                remain = count as i32;
            }
            for chan in &mut self.channels {
                crate::rendering::resample(
                    chan,
                    output_buffer,
                    offset as usize,
                    remain as usize,
                    &self.src.instruments,
                )
            }
            self.playback.tick_offset += remain;
            if self.playback.tick_offset == self.playback.tick_len {
                if self.sequence_tick() {
                    cnt = false;
                }
                self.playback.tick_offset = 0;
            }
            offset += remain;
            count -= remain as usize;
        }
        cnt
    }
    /// Calculate the song duration... Okay.
    pub fn calculate_song_duration(&mut self) -> i32 {
        if self.channels.is_empty() {
            0
        } else {
            let mut duration = 0;
            self.set_position(0);
            let mut song_end = false;
            while !song_end {
                duration += self.playback.tick_len;
                song_end = self.sequence_tick();
            }
            self.set_position(0);
            duration
        }
    }
    /// Mute a channel.
    pub fn mute_channel(&mut self, idx: usize) {
        if let Some(chan) = self.channels.get_mut(idx) {
            chan.mute = 1;
        }
    }
    /// Unmute all channels
    pub fn unmute_all(&mut self) {
        for chan in &mut self.channels {
            chan.mute = 0;
        }
    }
    /// Set some gainz.
    pub fn set_gain(&mut self, value: i32) {
        self.playback.gain = value;
    }
    fn set_position(&mut self, mut pos: i32) {
        if self.channels.is_empty() {
            return;
        }
        if pos >= self.src.song_length {
            pos = 0;
        }
        self.playback.break_pattern = pos;
        self.playback.next_row = 0;
        self.playback.tick = 1;
        self.playback.speed = 6;
        crate::rendering::set_tempo(125, &mut self.playback.tick_len, self.sample_rate);
        self.playback.pl_channel = -1;
        self.playback.pl_count = self.playback.pl_channel;
        self.playback.random_seed = 0xabcdef;
        for (i, chan) in self.channels.iter_mut().enumerate() {
            chan.id = i as u8;
            chan.assigned = 0;
            chan.instrument = 0;
            chan.volume = 0;
            match i & 0x3 {
                0 | 3 => {
                    chan.panning = 0;
                }
                1 | 2 => {
                    chan.panning = 127;
                }
                _ => {}
            }
        }
        self.sequence_tick();
        self.playback.tick_offset = 0;
    }
    fn sequence_row(&mut self) -> bool {
        let Self {
            sample_rate,
            channels,
            src,
            playback,
        } = self;
        let mut song_end = false;
        if playback.next_row < 0 {
            playback.break_pattern = playback.pattern + 1;
            playback.next_row = 0;
        }
        if playback.break_pattern >= 0 {
            if playback.break_pattern >= src.song_length {
                playback.next_row = 0;
                playback.break_pattern = playback.next_row;
            }
            if playback.break_pattern <= playback.pattern {
                song_end = true;
            }
            playback.pattern = playback.break_pattern;
            for chan in &mut *channels {
                chan.pl_row = 0
            }
            playback.break_pattern = -1;
        }
        playback.row = playback.next_row;
        playback.next_row = playback.row + 1;
        if playback.next_row >= 64 {
            playback.next_row = -1;
        }
        let mut pat_offset = ((i32::from(src.sequence[playback.pattern as usize]) * 64)
            + playback.row)
            * src.num_channels
            * 4;
        for chan in channels {
            let note = &mut chan.note;
            let pattern_data = src.pattern_data;
            note.key = (u16::from(pattern_data[pat_offset as usize]) & 0xf) << 8;
            note.key |= u16::from(pattern_data[(pat_offset + 1) as usize]);
            note.instrument = (pattern_data[(pat_offset + 2) as usize]) >> 4;
            note.instrument |= pattern_data[pat_offset as usize] & 0x10;
            let mut effect = pattern_data[(pat_offset + 2) as usize] & 0xf;
            let mut param = pattern_data[(pat_offset + 3) as usize];
            pat_offset += 4;
            if effect == 0xe {
                effect = 0x10 | param >> 4;
                param &= 0xf;
            }
            if effect == 0 && param > 0 {
                effect = 0xe;
            }
            note.effect = effect;
            note.param = param;
            crate::rendering::channel_row(chan, *sample_rate, src, playback);
        }
        song_end
    }
    fn sequence_tick(&mut self) -> bool {
        let mut song_end = false;
        self.playback.tick -= 1;
        if self.playback.tick <= 0 {
            self.playback.tick = self.playback.speed;
            song_end = self.sequence_row();
        } else {
            for chan in &mut self.channels {
                crate::rendering::channel_tick(
                    chan,
                    self.sample_rate,
                    self.playback.gain,
                    self.playback.c2_rate,
                    &mut self.playback.random_seed,
                    &self.src.instruments,
                );
            }
        }
        song_end
    }
}

/// An error that can happen when trying to initialize micromod
#[derive(Debug)]
pub enum InitError {
    /// Number of channels is incorrect
    ChannelNumIncorrect,
    /// Sampling rate is incorrect (below 8khz?)
    SamplingRateIncorrect,
}

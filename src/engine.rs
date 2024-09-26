use crate::{
    resample, sequence_tick,
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
        let mut mm = Engine {
            sample_rate,
            channels: vec![Channel::default(); num_channels as usize],
            src: ModSrc {
                instruments: Vec::new(),
                module_data: bytemuck::cast_slice(data),
                pattern_data,
                sequence,
                num_patterns: Default::default(),
                num_channels: num_channels.into(),
                song_length,
            },
            playback: PlaybackState::default(),
        };
        mm.src.num_patterns = crate::parse::calculate_num_patterns(data).into();
        let mut sample_data_offset = 1084 + mm.src.num_patterns * 64 * mm.src.num_channels * 4;
        let mut inst_idx = 1;
        // First instrument is an unused dummy instrument
        mm.src.instruments.push(Instrument::dummy());
        while inst_idx < 32 {
            let sample_length = u32::from(
                bytemuck::cast_slice(mm.src.module_data)
                    .read_u16_be(inst_idx * 30 + 12)
                    .unwrap(),
            ) * 2;
            let fine_tune = i32::from(mm.src.module_data[inst_idx * 30 + 14]) & 0xf;
            let fine_tune = ((fine_tune & 0x7) - (fine_tune & 0x8) + 8) as u8;
            let volume = i32::from(mm.src.module_data[inst_idx * 30 + 15]) & 0x7f;
            let volume = (if volume > 64 { 64 } else { volume }) as u8;

            let mut loop_start = u32::from(
                bytemuck::cast_slice(mm.src.module_data)
                    .read_u16_be(inst_idx * 30 + 16)
                    .unwrap(),
            ) * 2;
            let mut loop_length = u32::from(
                bytemuck::cast_slice(mm.src.module_data)
                    .read_u16_be(inst_idx * 30 + 18)
                    .unwrap(),
            ) * 2;

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
            let sample_data = &bytemuck::cast_slice::<u8, i8>(data)[sample_data_offset as usize..];
            sample_data_offset += sample_length as i32;
            inst_idx += 1;
            mm.src.instruments.push(Instrument {
                volume,
                fine_tune,
                loop_start: loop_start as usize,
                loop_length: loop_length as usize,
                sample_data,
            });
        }
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
                resample(
                    chan,
                    output_buffer,
                    offset as usize,
                    remain as usize,
                    &self.src.instruments,
                )
            }
            self.playback.tick_offset += remain;
            if self.playback.tick_offset == self.playback.tick_len {
                if sequence_tick(self) {
                    cnt = false;
                }
                self.playback.tick_offset = 0;
            }
            offset += remain;
            count -= remain as usize;
        }
        cnt
    }
    /// Calculate the length of the module file... In samples. Presumably.
    pub fn calculate_mod_file_len(&self) -> Option<u32> {
        let module_header = self.src.module_data;
        let numchan = u32::from(crate::parse::calculate_num_channels(bytemuck::cast_slice(
            module_header,
        ))?);
        let mut length = 1084
            + 4 * numchan
                * 64
                * u32::from(crate::parse::calculate_num_patterns(bytemuck::cast_slice(
                    module_header,
                )));
        let mut inst_idx = 1;
        while inst_idx < 32 {
            length += u32::from(
                bytemuck::cast_slice(module_header)
                    .read_u16_be(inst_idx * 30 + 12)
                    .unwrap(),
            ) * 2;
            inst_idx += 1;
        }
        Some(length)
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
                song_end = sequence_tick(self);
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
        crate::set_tempo(125, &mut self.playback.tick_len, self.sample_rate);
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
        sequence_tick(self);
        self.playback.tick_offset = 0;
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

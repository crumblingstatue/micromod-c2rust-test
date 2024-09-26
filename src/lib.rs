//! Experiment with converting [Micromod](https://github.com/martincameron/micromod) with
//! [c2rust](https://c2rust.com/).
//!
//! Safetyfication done manually

#![warn(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    clippy::cast_lossless,
    clippy::missing_const_for_fn,
    clippy::needless_pass_by_ref_mut
)]

mod consts;
#[cfg(test)]
mod tests;

use std::cmp::Ordering;

#[derive(Copy, Clone, Default)]
struct Channel {
    note: Note,
    period: u16,
    porta_period: u16,
    sample_offset: usize,
    sample_idx: usize,
    step: usize,
    volume: u8,
    panning: u8,
    fine_tune: u8,
    ampl: u8,
    mute: u8,
    id: u8,
    instrument: u8,
    assigned: u8,
    porta_speed: u8,
    pl_row: u8,
    fx_count: u8,
    vibrato_type: u8,
    vibrato_phase: u8,
    vibrato_speed: u8,
    vibrato_depth: u8,
    tremolo_type: u8,
    tremolo_phase: u8,
    tremolo_speed: u8,
    tremolo_depth: u8,
    tremolo_add: i8,
    vibrato_add: i8,
    arpeggio_add: i8,
}
#[derive(Copy, Clone, Default)]
struct Note {
    key: u16,
    instrument: u8,
    effect: u8,
    param: u8,
}
#[derive(Copy, Clone)]
struct Instrument<'src> {
    volume: u8,
    fine_tune: u8,
    loop_start: usize,
    loop_length: usize,
    sample_data: &'src [i8],
}

impl Instrument<'_> {
    const fn dummy() -> Self {
        Self {
            volume: 0,
            fine_tune: 0,
            loop_start: 0,
            loop_length: 0,
            sample_data: &[],
        }
    }
}

/// Immutable source data of the module
struct ModSrc<'src> {
    instruments: Vec<Instrument<'src>>,
    module_data: &'src [i8],
    pattern_data: &'src [u8],
    sequence: &'src [u8],
    num_patterns: i32,
    num_channels: i32,
    song_length: i32,
}

#[derive(Default)]
struct PlaybackState {
    gain: i32,
    c2_rate: i32,
    tick_len: i32,
    tick_offset: i32,
    pattern: i32,
    break_pattern: i32,
    row: i32,
    next_row: i32,
    tick: i32,
    speed: i32,
    pl_count: i32,
    pl_channel: i32,
    random_seed: i32,
}

/// A micromod decoding instance thingy.
pub struct MmC2r<'src> {
    sample_rate: i32,
    channels: [Channel; 16],
    src: ModSrc<'src>,
    playback: PlaybackState,
}

fn calculate_num_patterns(module_header: &[u8]) -> u16 {
    let mut num = 0;
    let mut i = 0;
    while i < 128 {
        let order_entry = u16::from(module_header[(952 + i) as usize]) & 0x7f;
        if order_entry >= num {
            num = order_entry + 1;
        }
        i += 1;
    }
    num
}

fn calculate_num_channels(module_header: &[u8]) -> Option<u16> {
    const MAX_CHANNELS: u16 = 16;
    let numchan: u16 = match (u16::from(module_header[1082]) << 8) | u16::from(module_header[1083])
    {
        // M.K.  M!K!     N.T.     FLT4
        0x4b2e | 0x4b21 | 0x542e | 0x5434 => 4,
        // xCHN
        0x484e => u16::from(module_header[1080] - 48),
        // xxCH
        0x4348 => u16::from(((module_header[1080] - 48) * 10) + (module_header[1081] - 48)),
        // Not recognised.
        _ => 0,
    };
    if numchan > MAX_CHANNELS {
        None
    } else {
        Some(numchan)
    }
}
fn set_tempo(tempo: i32, tick_len: &mut i32, sample_rate: i32) {
    *tick_len = ((sample_rate << 1) + (sample_rate >> 1)) / tempo;
}
fn update_frequency(chan: &mut Channel, sample_rate: i32, gain: i32, c2_rate: i32) {
    let mut period = i32::from(chan.period) + i32::from(chan.vibrato_add);
    period = (period * i32::from(consts::ARP_TUNING[chan.arpeggio_add as usize])) >> 11;
    period = (period >> 1) + (period & 1);
    if period < 14 {
        period = 6848;
    }
    let freq = (c2_rate * 428 / period) as u32;
    chan.step = (freq << 14).wrapping_div(sample_rate as u32) as usize;
    let mut volume = i32::from(chan.volume) + i32::from(chan.tremolo_add);
    volume = volume.clamp(0, 64);
    chan.ampl = ((volume * gain) >> 5) as u8;
}
fn tone_portamento(chan: &mut Channel) {
    let mut source = i32::from(chan.period);
    let dest = i32::from(chan.porta_period);
    match source.cmp(&dest) {
        Ordering::Less => {
            source += i32::from(chan.porta_speed);
            if source > dest {
                source = dest;
            }
        }
        Ordering::Equal => { /* Do absolutely nothing */ }
        Ordering::Greater => {
            source -= i32::from(chan.porta_speed);
            if source < dest {
                source = dest;
            }
        }
    }
    chan.period = source as u16;
}
fn volume_slide(chan: &mut Channel, param: i32) {
    let mut volume = i32::from(chan.volume) + (param >> 4) - (param & 0xf);
    volume = volume.clamp(0, 64);
    chan.volume = volume as u8;
}
fn waveform(phase: i32, type_: u8, random_seed: &mut i32) -> i32 {
    let mut amplitude: i32 = 0;
    match type_ & 0x3 {
        0 => {
            amplitude = i32::from(consts::SINE_TABLE[(phase & 0x1f) as usize]);
            if phase & 0x20 > 0 {
                amplitude = -amplitude;
            }
        }
        1 => {
            amplitude = 255 - (((phase + 0x20) & 0x3f) << 3);
        }
        2 => {
            amplitude = 255 - ((phase & 0x20) << 4);
        }
        3 => {
            amplitude = (*random_seed >> 20) - 255;
            *random_seed = (*random_seed * 65 + 17) & 0x1fffffff;
        }
        _ => {}
    }
    amplitude
}
fn vibrato(chan: &mut Channel, random_seed: &mut i32) {
    chan.vibrato_add = ((waveform(
        i32::from(chan.vibrato_phase),
        chan.vibrato_type,
        random_seed,
    ) * i32::from(chan.vibrato_depth))
        >> 7) as i8;
}
fn tremolo(chan: &mut Channel, random_seed: &mut i32) {
    chan.tremolo_add = ((waveform(
        i32::from(chan.tremolo_phase),
        chan.tremolo_type,
        random_seed,
    ) * i32::from(chan.tremolo_depth))
        >> 6) as i8;
}
fn trigger(channel: &mut Channel, instruments: &[Instrument]) {
    let ins = i32::from(channel.note.instrument);
    if ins > 0 && ins < 32 {
        channel.assigned = ins as u8;
        channel.sample_offset = 0;
        channel.fine_tune = instruments[ins as usize].fine_tune;
        channel.volume = instruments[ins as usize].volume;
        if instruments[ins as usize].loop_length > 0 && channel.instrument > 0 {
            channel.instrument = ins as u8;
        }
    }
    if channel.note.effect == 0x9 {
        channel.sample_offset = ((i32::from(channel.note.param) & 0xff) << 8) as usize;
    } else if channel.note.effect == 0x15 {
        channel.fine_tune = channel.note.param;
    }
    if channel.note.key > 0 {
        let period = (i32::from(channel.note.key)
            * i32::from(consts::FINE_TUNING[(i32::from(channel.fine_tune) & 0xf) as usize]))
            >> 11;
        channel.porta_period = ((period >> 1) + (period & 1)) as u16;
        if channel.note.effect != 0x3 && channel.note.effect != 0x5 {
            channel.instrument = channel.assigned;
            channel.period = channel.porta_period;
            channel.sample_idx = channel.sample_offset << 14;
            if channel.vibrato_type < 4 {
                channel.vibrato_phase = 0;
            }
            if channel.tremolo_type < 4 {
                channel.tremolo_phase = 0;
            }
        }
    }
}

trait SliceExt<T> {
    fn get_n<const N: usize>(&self, offset: usize) -> Option<&[T; N]>;
}

trait ByteSliceExt {
    fn read_u16_be(&self, offset: usize) -> Option<u16>;
}

impl<T> SliceExt<T> for [T] {
    fn get_n<const N: usize>(&self, offset: usize) -> Option<&[T; N]> {
        self.get(offset..).and_then(<[_]>::first_chunk::<N>)
    }
}

impl ByteSliceExt for [u8] {
    fn read_u16_be(&self, offset: usize) -> Option<u16> {
        self.get_n(offset).map(|arr| u16::from_be_bytes(*arr))
    }
}

fn channel_row(chan: &mut Channel, sample_rate: i32, src: &ModSrc, playback: &mut PlaybackState) {
    let effect = i32::from(chan.note.effect);
    let param = i32::from(chan.note.param);
    chan.fx_count = 0;
    chan.arpeggio_add = 0;
    chan.tremolo_add = 0;
    chan.vibrato_add = 0;
    if !(effect == 0x1d && param > 0) {
        trigger(chan, &src.instruments);
    }
    match effect {
        3 => {
            if param > 0 {
                chan.porta_speed = param as u8;
            }
        }
        4 => {
            if param & 0xf0 > 0 {
                chan.vibrato_speed = (param >> 4) as u8;
            }
            if param & 0xf > 0 {
                chan.vibrato_depth = (param & 0xf) as u8;
            }
            vibrato(chan, &mut playback.random_seed);
        }
        6 => {
            vibrato(chan, &mut playback.random_seed);
        }
        7 => {
            if param & 0xf0 > 0 {
                chan.tremolo_speed = (param >> 4) as u8;
            }
            if param & 0xf > 0 {
                chan.tremolo_depth = (param & 0xf) as u8;
            }
            tremolo(chan, &mut playback.random_seed);
        }
        8 => {
            if src.num_channels != 4 {
                chan.panning = (if param < 128 { param } else { 127 }) as u8;
            }
        }
        11 => {
            if playback.pl_count < 0 {
                playback.break_pattern = param;
                playback.next_row = 0;
            }
        }
        12 => {
            chan.volume = (if param > 64 { 64 } else { param }) as u8;
        }
        13 => {
            if playback.pl_count < 0 {
                if playback.break_pattern < 0 {
                    playback.break_pattern = playback.pattern + 1;
                }
                playback.next_row = (param >> 4) * 10 + (param & 0xf);
                if playback.next_row >= 64 {
                    playback.next_row = 0;
                }
            }
        }
        15 => {
            if param > 0 {
                if param < 32 {
                    playback.speed = param;
                    playback.tick = playback.speed;
                } else {
                    set_tempo(param, &mut playback.tick_len, sample_rate);
                }
            }
        }
        17 => {
            let period = i32::from(chan.period) - param;
            chan.period = (if period < 0 { 0 } else { period }) as u16;
        }
        18 => {
            let period = i32::from(chan.period) + param;
            chan.period = (if period > 65535 { 65535 } else { period }) as u16;
        }
        20 => {
            if param < 8 {
                chan.vibrato_type = param as u8;
            }
        }
        22 => {
            if param == 0 {
                chan.pl_row = playback.row as u8;
            }
            if i32::from(chan.pl_row) < playback.row && playback.break_pattern < 0 {
                if playback.pl_count < 0 {
                    playback.pl_count = param;
                    playback.pl_channel = i32::from(chan.id);
                }
                if playback.pl_channel == i32::from(chan.id) {
                    if playback.pl_count == 0 {
                        chan.pl_row = (playback.row + 1) as u8;
                    } else {
                        playback.next_row = i32::from(chan.pl_row);
                    }
                    playback.pl_count -= 1;
                }
            }
        }
        23 => {
            if param < 8 {
                chan.tremolo_type = param as u8;
            }
        }
        26 => {
            let volume = i32::from(chan.volume) + param;
            chan.volume = (if volume > 64 { 64 } else { volume }) as u8;
        }
        27 => {
            let volume = i32::from(chan.volume) - param;
            chan.volume = (if volume < 0 { 0 } else { volume }) as u8;
        }
        28 => {
            if param <= 0 {
                chan.volume = 0;
            }
        }
        30 => {
            playback.tick = playback.speed + playback.speed * param;
        }
        _ => {}
    }
    update_frequency(chan, sample_rate, playback.gain, playback.c2_rate);
}
fn channel_tick(
    chan: &mut Channel,
    sample_rate: i32,
    gain: i32,
    c2_rate: i32,
    random_seed: &mut i32,
    instruments: &[Instrument],
) {
    let effect = i32::from(chan.note.effect);
    let param = i32::from(chan.note.param);
    chan.fx_count = chan.fx_count.wrapping_add(1);
    match effect {
        1 => {
            let period = i32::from(chan.period) - param;
            chan.period = (if period < 0 { 0 } else { period }) as u16;
        }
        2 => {
            let period = i32::from(chan.period) + param;
            chan.period = (if period > 65535 { 65535 } else { period }) as u16;
        }
        3 => {
            tone_portamento(chan);
        }
        4 => {
            chan.vibrato_phase =
                (i32::from(chan.vibrato_phase) + i32::from(chan.vibrato_speed)) as u8;
            vibrato(chan, random_seed);
        }
        5 => {
            tone_portamento(chan);
            volume_slide(chan, param);
        }
        6 => {
            chan.vibrato_phase =
                (i32::from(chan.vibrato_phase) + i32::from(chan.vibrato_speed)) as u8;
            vibrato(chan, random_seed);
            volume_slide(chan, param);
        }
        7 => {
            chan.tremolo_phase =
                (i32::from(chan.tremolo_phase) + i32::from(chan.tremolo_speed)) as u8;
            tremolo(chan, random_seed);
        }
        10 => {
            volume_slide(chan, param);
        }
        14 => {
            if chan.fx_count > 2 {
                chan.fx_count = 0;
            }
            if chan.fx_count == 0 {
                chan.arpeggio_add = 0;
            }
            if chan.fx_count == 1 {
                chan.arpeggio_add = (param >> 4) as i8;
            }
            if chan.fx_count == 2 {
                chan.arpeggio_add = (param & 0xf) as i8;
            }
        }
        25 => {
            if i32::from(chan.fx_count) >= param {
                chan.fx_count = 0;
                chan.sample_idx = 0;
            }
        }
        28 => {
            if param == i32::from(chan.fx_count) {
                chan.volume = 0;
            }
        }
        29 => {
            if param == i32::from(chan.fx_count) {
                trigger(chan, instruments);
            }
        }
        _ => {}
    }
    if effect > 0 {
        update_frequency(chan, sample_rate, gain, c2_rate);
    }
}
fn sequence_row(
    MmC2r {
        sample_rate,
        channels,
        src,
        playback,
    }: &mut MmC2r,
) -> bool {
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
        let mut chan_idx = 0;
        while chan_idx < src.num_channels {
            channels[chan_idx as usize].pl_row = 0;
            chan_idx += 1;
        }
        playback.break_pattern = -1;
    }
    playback.row = playback.next_row;
    playback.next_row = playback.row + 1;
    if playback.next_row >= 64 {
        playback.next_row = -1;
    }
    let mut pat_offset = ((i32::from(src.sequence[playback.pattern as usize]) * 64) + playback.row)
        * src.num_channels
        * 4;
    let mut chan_idx = 0;
    while chan_idx < src.num_channels {
        let note = &mut (channels[chan_idx as usize]).note;
        let pattern_data = src.pattern_data;
        note.key = ((i32::from(pattern_data[pat_offset as usize]) & 0xf) << 8) as u16;
        note.key =
            (i32::from(note.key) | i32::from(pattern_data[(pat_offset + 1) as usize])) as u16;
        note.instrument = (i32::from(pattern_data[(pat_offset + 2) as usize]) >> 4) as u8;
        note.instrument = (i32::from(note.instrument)
            | i32::from(pattern_data[pat_offset as usize]) & 0x10) as u8;
        let mut effect = i32::from(pattern_data[(pat_offset + 2) as usize]) & 0xf;
        let mut param = i32::from(pattern_data[(pat_offset + 3) as usize]);
        pat_offset += 4;
        if effect == 0xe {
            effect = 0x10 | param >> 4;
            param &= 0xf;
        }
        if effect == 0 && param > 0 {
            effect = 0xe;
        }
        note.effect = effect as u8;
        note.param = param as u8;
        channel_row(
            &mut channels[chan_idx as usize],
            *sample_rate,
            src,
            playback,
        );
        chan_idx += 1;
    }
    song_end
}
fn sequence_tick(mm: &mut MmC2r) -> bool {
    let mut song_end = false;
    let mut chan_idx;
    mm.playback.tick -= 1;
    if mm.playback.tick <= 0 {
        mm.playback.tick = mm.playback.speed;
        song_end = sequence_row(mm);
    } else {
        chan_idx = 0;
        while chan_idx < mm.src.num_channels {
            channel_tick(
                &mut mm.channels[chan_idx as usize],
                mm.sample_rate,
                mm.playback.gain,
                mm.playback.c2_rate,
                &mut mm.playback.random_seed,
                &mm.src.instruments,
            );
            chan_idx += 1;
        }
    }
    song_end
}
fn resample(
    chan: &mut Channel,
    buf: &mut [i16],
    offset: usize,
    count: usize,
    instruments: &[Instrument],
) {
    let mut buf_idx: usize = offset << 1;
    let buf_end: usize = (offset + count) << 1;
    let mut sidx: usize = chan.sample_idx;
    let step: usize = chan.step;
    let llen = instruments[chan.instrument as usize].loop_length;
    let lep1 = (instruments[chan.instrument as usize].loop_start).wrapping_add(llen);
    let sdat = instruments[chan.instrument as usize].sample_data;
    let mut ampl: i16 = (if chan.mute == 0 {
        i32::from(chan.ampl)
    } else {
        0
    }) as i16;
    let lamp: i16 = ((i32::from(ampl) * (127 - i32::from(chan.panning))) >> 5) as i16;
    let ramp: i16 = ((i32::from(ampl) * i32::from(chan.panning)) >> 5) as i16;
    while buf_idx < buf_end {
        if sidx >= lep1 {
            if llen <= 16384 {
                sidx = lep1;
                break;
            }
            while sidx >= lep1 {
                sidx = sidx.wrapping_sub(llen);
            }
        }
        let mut epos = sidx.wrapping_add((buf_end.wrapping_sub(buf_idx) >> 1).wrapping_mul(step));
        if lamp != 0 || ramp != 0 {
            if epos > lep1 {
                epos = lep1;
            }
            if lamp != 0 && ramp != 0 {
                while sidx < epos {
                    ampl = i16::from(sdat[sidx >> 14]);
                    let idx = buf_idx;
                    buf_idx = buf_idx.wrapping_add(1);
                    let sample = &mut buf[idx];
                    *sample =
                        (i32::from(*sample) + ((i32::from(ampl) * i32::from(lamp)) >> 2)) as i16;
                    let idx = buf_idx;
                    buf_idx = buf_idx.wrapping_add(1);
                    let sample = &mut buf[idx];
                    *sample =
                        (i32::from(*sample) + ((i32::from(ampl) * i32::from(ramp)) >> 2)) as i16;
                    sidx = sidx.wrapping_add(step);
                }
            } else {
                if ramp != 0 {
                    buf_idx = buf_idx.wrapping_add(1);
                }
                while sidx < epos {
                    let sample = &mut buf[buf_idx];
                    *sample =
                        (i32::from(*sample) + i32::from(sdat[sidx >> 14]) * i32::from(ampl)) as i16;
                    buf_idx = buf_idx.wrapping_add(2);
                    sidx = sidx.wrapping_add(step);
                }
                buf_idx &= -2_i32 as usize;
            }
        } else {
            buf_idx = buf_end;
            sidx = epos;
        }
    }
    chan.sample_idx = sidx;
}

/// Get a nice version string. I guess.
pub fn version() -> &'static str {
    consts::MICROMOD_VERSION
}

/// An error that can happen when trying to initialize micromod
#[derive(Debug)]
pub enum InitError {
    /// Number of channels is incorrect
    ChannelNumIncorrect,
    /// Sampling rate is incorrect (below 8khz?)
    SamplingRateIncorrect,
}

impl MmC2r<'_> {
    /// Create a new micromod decoder apparatus.
    pub fn new(data: &[u8], sample_rate: i32) -> Result<MmC2r, InitError> {
        let num_channels = match calculate_num_channels(data) {
            Some(num_channels) => num_channels,
            None => return Err(InitError::ChannelNumIncorrect),
        };
        if sample_rate < 8000 {
            return Err(InitError::SamplingRateIncorrect);
        }
        let song_length = i32::from(data[950]) & 0x7f;
        let sequence = &data[952..];
        let pattern_data = &data[1084..];
        let mut mm = MmC2r {
            sample_rate,
            channels: Default::default(),
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
        mm.src.num_patterns = calculate_num_patterns(data).into();
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
        mm.mute_channel(-1);
        micromod_set_position(0, &mut mm);
        Ok(mm)
    }
    /// Fill a buffer with delicious samples
    pub fn get_audio(&mut self, output_buffer: &mut [i16], mut count: usize) -> bool {
        if self.src.num_channels <= 0 {
            return false;
        }
        let mut offset = 0;
        let mut cnt = true;
        while count > 0 {
            let mut remain = self.playback.tick_len - self.playback.tick_offset;
            if remain > count as i32 {
                remain = count as i32;
            }
            let mut chan_idx = 0;
            while chan_idx < self.src.num_channels {
                resample(
                    &mut self.channels[chan_idx as usize],
                    output_buffer,
                    offset as usize,
                    remain as usize,
                    &self.src.instruments,
                );
                chan_idx += 1;
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
    pub fn calculate_mod_file_len(&self) -> Option<i32> {
        let module_header = self.src.module_data;
        let numchan = i32::from(calculate_num_channels(bytemuck::cast_slice(module_header))?);
        let mut length = 1084
            + 4 * numchan
                * 64
                * i32::from(calculate_num_patterns(bytemuck::cast_slice(module_header)));
        let mut inst_idx = 1;
        while inst_idx < 32 {
            length += i32::from(
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
        let mut duration = 0;
        if self.src.num_channels > 0 {
            micromod_set_position(0, self);
            let mut song_end = false;
            while !song_end {
                duration += self.playback.tick_len;
                song_end = sequence_tick(self);
            }
            micromod_set_position(0, self);
        }
        duration
    }
    /// Mute a channel.
    pub fn mute_channel(&mut self, channel: i32) -> i32 {
        if channel < 0 {
            let mut chan_idx = 0;
            while chan_idx < self.src.num_channels {
                self.channels[chan_idx as usize].mute = 0;
                chan_idx += 1;
            }
        } else if channel < self.src.num_channels {
            self.channels[channel as usize].mute = 1;
        }
        self.src.num_channels
    }
    /// Set some gainz.
    pub fn set_gain(&mut self, value: i32) {
        self.playback.gain = value;
    }
}

fn micromod_set_position(mut pos: i32, mm: &mut MmC2r) {
    if mm.src.num_channels <= 0 {
        return;
    }
    if pos >= mm.src.song_length {
        pos = 0;
    }
    mm.playback.break_pattern = pos;
    mm.playback.next_row = 0;
    mm.playback.tick = 1;
    mm.playback.speed = 6;
    set_tempo(125, &mut mm.playback.tick_len, mm.sample_rate);
    mm.playback.pl_channel = -1;
    mm.playback.pl_count = mm.playback.pl_channel;
    mm.playback.random_seed = 0xabcdef;
    let mut chan_idx = 0;
    while chan_idx < mm.src.num_channels {
        let chan = &mut mm.channels[chan_idx as usize];
        chan.id = chan_idx as u8;
        chan.assigned = 0;
        chan.instrument = 0;
        chan.volume = 0;
        match chan_idx & 0x3 {
            0 | 3 => {
                chan.panning = 0;
            }
            1 | 2 => {
                chan.panning = 127;
            }
            _ => {}
        }
        chan_idx += 1;
    }
    sequence_tick(mm);
    mm.playback.tick_offset = 0;
}

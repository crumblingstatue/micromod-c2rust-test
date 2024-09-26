use {
    crate::{
        consts,
        types::{Channel, Instrument, ModSrc, PlaybackState},
    },
    std::cmp::Ordering,
};

pub(crate) fn set_tempo(tempo: i32, tick_len: &mut i32, sample_rate: i32) {
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
fn waveform(phase: u8, type_: u8, random_seed: &mut i32) -> i16 {
    let mut amplitude: i16 = 0;
    match type_ & 0x3 {
        0 => {
            amplitude = i16::from(consts::SINE_TABLE[(phase & 0x1f) as usize]);
            if phase & 0x20 > 0 {
                amplitude = -amplitude;
            }
        }
        1 => {
            amplitude = 255 - (((i16::from(phase) + 0x20) & 0x3f) << 3);
        }
        2 => {
            amplitude = 255 - ((i16::from(phase) & 0x20) << 4);
        }
        3 => {
            amplitude = (*random_seed as i16 >> 10) - 255;
            *random_seed = (*random_seed * 65 + 17) & 0x1fffffff;
        }
        _ => {}
    }
    amplitude
}
fn vibrato(chan: &mut Channel, random_seed: &mut i32) {
    let amp = waveform(chan.vibrato_phase, chan.vibrato_type, random_seed);
    chan.vibrato_add = ((amp * i16::from(chan.vibrato_depth)) >> 7) as i8;
}
fn tremolo(chan: &mut Channel, random_seed: &mut i32) {
    let amp = waveform(chan.tremolo_phase, chan.tremolo_type, random_seed);
    chan.tremolo_add = ((amp * i16::from(chan.tremolo_depth)) >> 6) as i8;
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

pub(crate) fn channel_row(
    chan: &mut Channel,
    sample_rate: i32,
    src: &ModSrc,
    playback: &mut PlaybackState,
) {
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
pub(crate) fn channel_tick(
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
pub(crate) fn resample(
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
    let mut ampl = if chan.mute == 0 { chan.ampl } else { 0 };
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
                    ampl = sdat[sidx >> 14] as u8;
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

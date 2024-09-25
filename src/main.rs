#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![register_tool(c2rust)]
#![feature(register_tool)]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct channel {
    pub note: note,
    pub period: libc::c_ushort,
    pub porta_period: libc::c_ushort,
    pub sample_offset: libc::c_ulong,
    pub sample_idx: libc::c_ulong,
    pub step: libc::c_ulong,
    pub volume: libc::c_uchar,
    pub panning: libc::c_uchar,
    pub fine_tune: libc::c_uchar,
    pub ampl: libc::c_uchar,
    pub mute: libc::c_uchar,
    pub id: libc::c_uchar,
    pub instrument: libc::c_uchar,
    pub assigned: libc::c_uchar,
    pub porta_speed: libc::c_uchar,
    pub pl_row: libc::c_uchar,
    pub fx_count: libc::c_uchar,
    pub vibrato_type: libc::c_uchar,
    pub vibrato_phase: libc::c_uchar,
    pub vibrato_speed: libc::c_uchar,
    pub vibrato_depth: libc::c_uchar,
    pub tremolo_type: libc::c_uchar,
    pub tremolo_phase: libc::c_uchar,
    pub tremolo_speed: libc::c_uchar,
    pub tremolo_depth: libc::c_uchar,
    pub tremolo_add: libc::c_schar,
    pub vibrato_add: libc::c_schar,
    pub arpeggio_add: libc::c_schar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct note {
    pub key: libc::c_ushort,
    pub instrument: libc::c_uchar,
    pub effect: libc::c_uchar,
    pub param: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct instrument {
    pub volume: libc::c_uchar,
    pub fine_tune: libc::c_uchar,
    pub loop_start: libc::c_ulong,
    pub loop_length: libc::c_ulong,
    pub sample_data: *mut libc::c_schar,
}
static mut MICROMOD_VERSION: *const libc::c_char =
    b"Micromod Protracker replay 20180625 (c)mumart@gmail.com\0" as *const u8
        as *const libc::c_char;
static mut fine_tuning: [libc::c_ushort; 16] = [
    4340 as libc::c_int as libc::c_ushort,
    4308 as libc::c_int as libc::c_ushort,
    4277 as libc::c_int as libc::c_ushort,
    4247 as libc::c_int as libc::c_ushort,
    4216 as libc::c_int as libc::c_ushort,
    4186 as libc::c_int as libc::c_ushort,
    4156 as libc::c_int as libc::c_ushort,
    4126 as libc::c_int as libc::c_ushort,
    4096 as libc::c_int as libc::c_ushort,
    4067 as libc::c_int as libc::c_ushort,
    4037 as libc::c_int as libc::c_ushort,
    4008 as libc::c_int as libc::c_ushort,
    3979 as libc::c_int as libc::c_ushort,
    3951 as libc::c_int as libc::c_ushort,
    3922 as libc::c_int as libc::c_ushort,
    3894 as libc::c_int as libc::c_ushort,
];
static mut arp_tuning: [libc::c_ushort; 16] = [
    4096 as libc::c_int as libc::c_ushort,
    3866 as libc::c_int as libc::c_ushort,
    3649 as libc::c_int as libc::c_ushort,
    3444 as libc::c_int as libc::c_ushort,
    3251 as libc::c_int as libc::c_ushort,
    3069 as libc::c_int as libc::c_ushort,
    2896 as libc::c_int as libc::c_ushort,
    2734 as libc::c_int as libc::c_ushort,
    2580 as libc::c_int as libc::c_ushort,
    2435 as libc::c_int as libc::c_ushort,
    2299 as libc::c_int as libc::c_ushort,
    2170 as libc::c_int as libc::c_ushort,
    2048 as libc::c_int as libc::c_ushort,
    1933 as libc::c_int as libc::c_ushort,
    1825 as libc::c_int as libc::c_ushort,
    1722 as libc::c_int as libc::c_ushort,
];
static mut sine_table: [libc::c_uchar; 32] = [
    0 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
    49 as libc::c_int as libc::c_uchar,
    74 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    141 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
    180 as libc::c_int as libc::c_uchar,
    197 as libc::c_int as libc::c_uchar,
    212 as libc::c_int as libc::c_uchar,
    224 as libc::c_int as libc::c_uchar,
    235 as libc::c_int as libc::c_uchar,
    244 as libc::c_int as libc::c_uchar,
    250 as libc::c_int as libc::c_uchar,
    253 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    253 as libc::c_int as libc::c_uchar,
    250 as libc::c_int as libc::c_uchar,
    244 as libc::c_int as libc::c_uchar,
    235 as libc::c_int as libc::c_uchar,
    224 as libc::c_int as libc::c_uchar,
    212 as libc::c_int as libc::c_uchar,
    197 as libc::c_int as libc::c_uchar,
    180 as libc::c_int as libc::c_uchar,
    161 as libc::c_int as libc::c_uchar,
    141 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    74 as libc::c_int as libc::c_uchar,
    49 as libc::c_int as libc::c_uchar,
    24 as libc::c_int as libc::c_uchar,
];
static mut module_data: *mut libc::c_schar = 0 as *const libc::c_schar as *mut libc::c_schar;
static mut pattern_data: *mut libc::c_uchar = 0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut sequence: *mut libc::c_uchar = 0 as *const libc::c_uchar as *mut libc::c_uchar;
static mut song_length: libc::c_long = 0;
static mut restart: libc::c_long = 0;
static mut num_patterns: libc::c_long = 0;
static mut num_channels: libc::c_long = 0;
static mut instruments: [instrument; 32] = [instrument {
    volume: 0,
    fine_tune: 0,
    loop_start: 0,
    loop_length: 0,
    sample_data: 0 as *const libc::c_schar as *mut libc::c_schar,
}; 32];
static mut sample_rate: libc::c_long = 0;
static mut gain: libc::c_long = 0;
static mut c2_rate: libc::c_long = 0;
static mut tick_len: libc::c_long = 0;
static mut tick_offset: libc::c_long = 0;
static mut pattern: libc::c_long = 0;
static mut break_pattern: libc::c_long = 0;
static mut row: libc::c_long = 0;
static mut next_row: libc::c_long = 0;
static mut tick: libc::c_long = 0;
static mut speed: libc::c_long = 0;
static mut pl_count: libc::c_long = 0;
static mut pl_channel: libc::c_long = 0;
static mut random_seed: libc::c_long = 0;
static mut channels: [channel; 16] = [channel {
    note: note {
        key: 0,
        instrument: 0,
        effect: 0,
        param: 0,
    },
    period: 0,
    porta_period: 0,
    sample_offset: 0,
    sample_idx: 0,
    step: 0,
    volume: 0,
    panning: 0,
    fine_tune: 0,
    ampl: 0,
    mute: 0,
    id: 0,
    instrument: 0,
    assigned: 0,
    porta_speed: 0,
    pl_row: 0,
    fx_count: 0,
    vibrato_type: 0,
    vibrato_phase: 0,
    vibrato_speed: 0,
    vibrato_depth: 0,
    tremolo_type: 0,
    tremolo_phase: 0,
    tremolo_speed: 0,
    tremolo_depth: 0,
    tremolo_add: 0,
    vibrato_add: 0,
    arpeggio_add: 0,
}; 16];
unsafe extern "C" fn calculate_num_patterns(mut module_header: *mut libc::c_schar) -> libc::c_long {
    let mut num_patterns_0: libc::c_long = 0;
    let mut order_entry: libc::c_long = 0;
    let mut pattern_0: libc::c_long = 0;
    num_patterns_0 = 0 as libc::c_int as libc::c_long;
    pattern_0 = 0 as libc::c_int as libc::c_long;
    while pattern_0 < 128 as libc::c_int as libc::c_long {
        order_entry = (*module_header
            .offset((952 as libc::c_int as libc::c_long + pattern_0) as isize)
            as libc::c_int
            & 0x7f as libc::c_int) as libc::c_long;
        if order_entry >= num_patterns_0 {
            num_patterns_0 = order_entry + 1 as libc::c_int as libc::c_long;
        }
        pattern_0 += 1;
    }
    return num_patterns_0;
}
unsafe extern "C" fn calculate_num_channels(mut module_header: *mut libc::c_schar) -> libc::c_long {
    let mut numchan: libc::c_long = 0;
    let mut current_block_3: u64;
    match (*module_header.offset(1082 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
        | *module_header.offset(1083 as libc::c_int as isize) as libc::c_int
    {
        19233 => {
            current_block_3 = 4379976358253192308;
        }
        21550 => {
            current_block_3 = 4379976358253192308;
        }
        19246 | 21556 => {
            current_block_3 = 5412093109544641453;
        }
        18510 => {
            numchan = (*module_header.offset(1080 as libc::c_int as isize) as libc::c_int
                - 48 as libc::c_int) as libc::c_long;
            current_block_3 = 3276175668257526147;
        }
        17224 => {
            numchan = ((*module_header.offset(1080 as libc::c_int as isize) as libc::c_int
                - 48 as libc::c_int)
                * 10 as libc::c_int
                + (*module_header.offset(1081 as libc::c_int as isize) as libc::c_int
                    - 48 as libc::c_int)) as libc::c_long;
            current_block_3 = 3276175668257526147;
        }
        _ => {
            numchan = 0 as libc::c_int as libc::c_long;
            current_block_3 = 3276175668257526147;
        }
    }
    match current_block_3 {
        4379976358253192308 => {
            current_block_3 = 5412093109544641453;
        }
        _ => {}
    }
    match current_block_3 {
        5412093109544641453 => {
            numchan = 4 as libc::c_int as libc::c_long;
        }
        _ => {}
    }
    if numchan > 16 as libc::c_int as libc::c_long {
        numchan = 0 as libc::c_int as libc::c_long;
    }
    return numchan;
}
unsafe extern "C" fn unsigned_short_big_endian(
    mut buf: *mut libc::c_schar,
    mut offset: libc::c_long,
) -> libc::c_long {
    return ((*buf.offset(offset as isize) as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int
        | *buf.offset((offset + 1 as libc::c_int as libc::c_long) as isize) as libc::c_int
            & 0xff as libc::c_int) as libc::c_long;
}
unsafe extern "C" fn set_tempo(mut tempo: libc::c_long) {
    tick_len = ((sample_rate << 1 as libc::c_int) + (sample_rate >> 1 as libc::c_int)) / tempo;
}
unsafe extern "C" fn update_frequency(mut chan: *mut channel) {
    let mut period: libc::c_long = 0;
    let mut volume: libc::c_long = 0;
    let mut freq: libc::c_ulong = 0;
    period = ((*chan).period as libc::c_int + (*chan).vibrato_add as libc::c_int) as libc::c_long;
    period =
        period * arp_tuning[(*chan).arpeggio_add as usize] as libc::c_long >> 11 as libc::c_int;
    period = (period >> 1 as libc::c_int) + (period & 1 as libc::c_int as libc::c_long);
    if period < 14 as libc::c_int as libc::c_long {
        period = 6848 as libc::c_int as libc::c_long;
    }
    freq = (c2_rate * 428 as libc::c_int as libc::c_long / period) as libc::c_ulong;
    (*chan).step = (freq << 14 as libc::c_int).wrapping_div(sample_rate as libc::c_ulong);
    volume = ((*chan).volume as libc::c_int + (*chan).tremolo_add as libc::c_int) as libc::c_long;
    if volume > 64 as libc::c_int as libc::c_long {
        volume = 64 as libc::c_int as libc::c_long;
    }
    if volume < 0 as libc::c_int as libc::c_long {
        volume = 0 as libc::c_int as libc::c_long;
    }
    (*chan).ampl = (volume * gain >> 5 as libc::c_int) as libc::c_uchar;
}
unsafe extern "C" fn tone_portamento(mut chan: *mut channel) {
    let mut source: libc::c_long = 0;
    let mut dest: libc::c_long = 0;
    source = (*chan).period as libc::c_long;
    dest = (*chan).porta_period as libc::c_long;
    if source < dest {
        source += (*chan).porta_speed as libc::c_long;
        if source > dest {
            source = dest;
        }
    } else if source > dest {
        source -= (*chan).porta_speed as libc::c_long;
        if source < dest {
            source = dest;
        }
    }
    (*chan).period = source as libc::c_ushort;
}
unsafe extern "C" fn volume_slide(mut chan: *mut channel, mut param: libc::c_long) {
    let mut volume: libc::c_long = 0;
    volume = (*chan).volume as libc::c_long + (param >> 4 as libc::c_int)
        - (param & 0xf as libc::c_int as libc::c_long);
    if volume < 0 as libc::c_int as libc::c_long {
        volume = 0 as libc::c_int as libc::c_long;
    }
    if volume > 64 as libc::c_int as libc::c_long {
        volume = 64 as libc::c_int as libc::c_long;
    }
    (*chan).volume = volume as libc::c_uchar;
}
unsafe extern "C" fn waveform(mut phase: libc::c_long, mut type_0: libc::c_long) -> libc::c_long {
    let mut amplitude: libc::c_long = 0 as libc::c_int as libc::c_long;
    match type_0 & 0x3 as libc::c_int as libc::c_long {
        0 => {
            amplitude =
                sine_table[(phase & 0x1f as libc::c_int as libc::c_long) as usize] as libc::c_long;
            if phase & 0x20 as libc::c_int as libc::c_long > 0 as libc::c_int as libc::c_long {
                amplitude = -amplitude;
            }
        }
        1 => {
            amplitude = 255 as libc::c_int as libc::c_long
                - ((phase + 0x20 as libc::c_int as libc::c_long
                    & 0x3f as libc::c_int as libc::c_long)
                    << 3 as libc::c_int);
        }
        2 => {
            amplitude = 255 as libc::c_int as libc::c_long
                - ((phase & 0x20 as libc::c_int as libc::c_long) << 4 as libc::c_int);
        }
        3 => {
            amplitude = (random_seed >> 20 as libc::c_int) - 255 as libc::c_int as libc::c_long;
            random_seed = random_seed * 65 as libc::c_int as libc::c_long
                + 17 as libc::c_int as libc::c_long
                & 0x1fffffff as libc::c_int as libc::c_long;
        }
        _ => {}
    }
    return amplitude;
}
unsafe extern "C" fn vibrato(mut chan: *mut channel) {
    (*chan).vibrato_add = (waveform(
        (*chan).vibrato_phase as libc::c_long,
        (*chan).vibrato_type as libc::c_long,
    ) * (*chan).vibrato_depth as libc::c_long
        >> 7 as libc::c_int) as libc::c_schar;
}
unsafe extern "C" fn tremolo(mut chan: *mut channel) {
    (*chan).tremolo_add = (waveform(
        (*chan).tremolo_phase as libc::c_long,
        (*chan).tremolo_type as libc::c_long,
    ) * (*chan).tremolo_depth as libc::c_long
        >> 6 as libc::c_int) as libc::c_schar;
}
unsafe extern "C" fn trigger(mut channel: *mut channel) {
    let mut period: libc::c_long = 0;
    let mut ins: libc::c_long = 0;
    ins = (*channel).note.instrument as libc::c_long;
    if ins > 0 as libc::c_int as libc::c_long && ins < 32 as libc::c_int as libc::c_long {
        (*channel).assigned = ins as libc::c_uchar;
        (*channel).sample_offset = 0 as libc::c_int as libc::c_ulong;
        (*channel).fine_tune = instruments[ins as usize].fine_tune;
        (*channel).volume = instruments[ins as usize].volume;
        if instruments[ins as usize].loop_length > 0 as libc::c_int as libc::c_ulong
            && (*channel).instrument as libc::c_int > 0 as libc::c_int
        {
            (*channel).instrument = ins as libc::c_uchar;
        }
    }
    if (*channel).note.effect as libc::c_int == 0x9 as libc::c_int {
        (*channel).sample_offset = (((*channel).note.param as libc::c_int & 0xff as libc::c_int)
            << 8 as libc::c_int) as libc::c_ulong;
    } else if (*channel).note.effect as libc::c_int == 0x15 as libc::c_int {
        (*channel).fine_tune = (*channel).note.param;
    }
    if (*channel).note.key as libc::c_int > 0 as libc::c_int {
        period = ((*channel).note.key as libc::c_int
            * fine_tuning[((*channel).fine_tune as libc::c_int & 0xf as libc::c_int) as usize]
                as libc::c_int
            >> 11 as libc::c_int) as libc::c_long;
        (*channel).porta_period = ((period >> 1 as libc::c_int)
            + (period & 1 as libc::c_int as libc::c_long))
            as libc::c_ushort;
        if (*channel).note.effect as libc::c_int != 0x3 as libc::c_int
            && (*channel).note.effect as libc::c_int != 0x5 as libc::c_int
        {
            (*channel).instrument = (*channel).assigned;
            (*channel).period = (*channel).porta_period;
            (*channel).sample_idx = (*channel).sample_offset << 14 as libc::c_int;
            if ((*channel).vibrato_type as libc::c_int) < 4 as libc::c_int {
                (*channel).vibrato_phase = 0 as libc::c_int as libc::c_uchar;
            }
            if ((*channel).tremolo_type as libc::c_int) < 4 as libc::c_int {
                (*channel).tremolo_phase = 0 as libc::c_int as libc::c_uchar;
            }
        }
    }
}
unsafe extern "C" fn channel_row(mut chan: *mut channel) {
    let mut effect: libc::c_long = 0;
    let mut param: libc::c_long = 0;
    let mut volume: libc::c_long = 0;
    let mut period: libc::c_long = 0;
    effect = (*chan).note.effect as libc::c_long;
    param = (*chan).note.param as libc::c_long;
    let ref mut fresh0 = (*chan).fx_count;
    *fresh0 = 0 as libc::c_int as libc::c_uchar;
    let ref mut fresh1 = (*chan).arpeggio_add;
    *fresh1 = *fresh0 as libc::c_schar;
    let ref mut fresh2 = (*chan).tremolo_add;
    *fresh2 = *fresh1;
    (*chan).vibrato_add = *fresh2;
    if !(effect == 0x1d as libc::c_int as libc::c_long && param > 0 as libc::c_int as libc::c_long)
    {
        trigger(chan);
    }
    match effect {
        3 => {
            if param > 0 as libc::c_int as libc::c_long {
                (*chan).porta_speed = param as libc::c_uchar;
            }
        }
        4 => {
            if param & 0xf0 as libc::c_int as libc::c_long > 0 as libc::c_int as libc::c_long {
                (*chan).vibrato_speed = (param >> 4 as libc::c_int) as libc::c_uchar;
            }
            if param & 0xf as libc::c_int as libc::c_long > 0 as libc::c_int as libc::c_long {
                (*chan).vibrato_depth =
                    (param & 0xf as libc::c_int as libc::c_long) as libc::c_uchar;
            }
            vibrato(chan);
        }
        6 => {
            vibrato(chan);
        }
        7 => {
            if param & 0xf0 as libc::c_int as libc::c_long > 0 as libc::c_int as libc::c_long {
                (*chan).tremolo_speed = (param >> 4 as libc::c_int) as libc::c_uchar;
            }
            if param & 0xf as libc::c_int as libc::c_long > 0 as libc::c_int as libc::c_long {
                (*chan).tremolo_depth =
                    (param & 0xf as libc::c_int as libc::c_long) as libc::c_uchar;
            }
            tremolo(chan);
        }
        8 => {
            if num_channels != 4 as libc::c_int as libc::c_long {
                (*chan).panning = (if param < 128 as libc::c_int as libc::c_long {
                    param
                } else {
                    127 as libc::c_int as libc::c_long
                }) as libc::c_uchar;
            }
        }
        11 => {
            if pl_count < 0 as libc::c_int as libc::c_long {
                break_pattern = param;
                next_row = 0 as libc::c_int as libc::c_long;
            }
        }
        12 => {
            (*chan).volume = (if param > 64 as libc::c_int as libc::c_long {
                64 as libc::c_int as libc::c_long
            } else {
                param
            }) as libc::c_uchar;
        }
        13 => {
            if pl_count < 0 as libc::c_int as libc::c_long {
                if break_pattern < 0 as libc::c_int as libc::c_long {
                    break_pattern = pattern + 1 as libc::c_int as libc::c_long;
                }
                next_row = (param >> 4 as libc::c_int) * 10 as libc::c_int as libc::c_long
                    + (param & 0xf as libc::c_int as libc::c_long);
                if next_row >= 64 as libc::c_int as libc::c_long {
                    next_row = 0 as libc::c_int as libc::c_long;
                }
            }
        }
        15 => {
            if param > 0 as libc::c_int as libc::c_long {
                if param < 32 as libc::c_int as libc::c_long {
                    speed = param;
                    tick = speed;
                } else {
                    set_tempo(param);
                }
            }
        }
        17 => {
            period = (*chan).period as libc::c_long - param;
            (*chan).period = (if period < 0 as libc::c_int as libc::c_long {
                0 as libc::c_int as libc::c_long
            } else {
                period
            }) as libc::c_ushort;
        }
        18 => {
            period = (*chan).period as libc::c_long + param;
            (*chan).period = (if period > 65535 as libc::c_int as libc::c_long {
                65535 as libc::c_int as libc::c_long
            } else {
                period
            }) as libc::c_ushort;
        }
        20 => {
            if param < 8 as libc::c_int as libc::c_long {
                (*chan).vibrato_type = param as libc::c_uchar;
            }
        }
        22 => {
            if param == 0 as libc::c_int as libc::c_long {
                (*chan).pl_row = row as libc::c_uchar;
            }
            if ((*chan).pl_row as libc::c_long) < row
                && break_pattern < 0 as libc::c_int as libc::c_long
            {
                if pl_count < 0 as libc::c_int as libc::c_long {
                    pl_count = param;
                    pl_channel = (*chan).id as libc::c_long;
                }
                if pl_channel == (*chan).id as libc::c_long {
                    if pl_count == 0 as libc::c_int as libc::c_long {
                        (*chan).pl_row = (row + 1 as libc::c_int as libc::c_long) as libc::c_uchar;
                    } else {
                        next_row = (*chan).pl_row as libc::c_long;
                    }
                    pl_count -= 1;
                }
            }
        }
        23 => {
            if param < 8 as libc::c_int as libc::c_long {
                (*chan).tremolo_type = param as libc::c_uchar;
            }
        }
        26 => {
            volume = (*chan).volume as libc::c_long + param;
            (*chan).volume = (if volume > 64 as libc::c_int as libc::c_long {
                64 as libc::c_int as libc::c_long
            } else {
                volume
            }) as libc::c_uchar;
        }
        27 => {
            volume = (*chan).volume as libc::c_long - param;
            (*chan).volume = (if volume < 0 as libc::c_int as libc::c_long {
                0 as libc::c_int as libc::c_long
            } else {
                volume
            }) as libc::c_uchar;
        }
        28 => {
            if param <= 0 as libc::c_int as libc::c_long {
                (*chan).volume = 0 as libc::c_int as libc::c_uchar;
            }
        }
        30 => {
            tick = speed + speed * param;
        }
        _ => {}
    }
    update_frequency(chan);
}
unsafe extern "C" fn channel_tick(mut chan: *mut channel) {
    let mut effect: libc::c_long = 0;
    let mut param: libc::c_long = 0;
    let mut period: libc::c_long = 0;
    effect = (*chan).note.effect as libc::c_long;
    param = (*chan).note.param as libc::c_long;
    let ref mut fresh3 = (*chan).fx_count;
    *fresh3 = (*fresh3).wrapping_add(1);
    match effect {
        1 => {
            period = (*chan).period as libc::c_long - param;
            (*chan).period = (if period < 0 as libc::c_int as libc::c_long {
                0 as libc::c_int as libc::c_long
            } else {
                period
            }) as libc::c_ushort;
        }
        2 => {
            period = (*chan).period as libc::c_long + param;
            (*chan).period = (if period > 65535 as libc::c_int as libc::c_long {
                65535 as libc::c_int as libc::c_long
            } else {
                period
            }) as libc::c_ushort;
        }
        3 => {
            tone_portamento(chan);
        }
        4 => {
            let ref mut fresh4 = (*chan).vibrato_phase;
            *fresh4 =
                (*fresh4 as libc::c_int + (*chan).vibrato_speed as libc::c_int) as libc::c_uchar;
            vibrato(chan);
        }
        5 => {
            tone_portamento(chan);
            volume_slide(chan, param);
        }
        6 => {
            let ref mut fresh5 = (*chan).vibrato_phase;
            *fresh5 =
                (*fresh5 as libc::c_int + (*chan).vibrato_speed as libc::c_int) as libc::c_uchar;
            vibrato(chan);
            volume_slide(chan, param);
        }
        7 => {
            let ref mut fresh6 = (*chan).tremolo_phase;
            *fresh6 =
                (*fresh6 as libc::c_int + (*chan).tremolo_speed as libc::c_int) as libc::c_uchar;
            tremolo(chan);
        }
        10 => {
            volume_slide(chan, param);
        }
        14 => {
            if (*chan).fx_count as libc::c_int > 2 as libc::c_int {
                (*chan).fx_count = 0 as libc::c_int as libc::c_uchar;
            }
            if (*chan).fx_count as libc::c_int == 0 as libc::c_int {
                (*chan).arpeggio_add = 0 as libc::c_int as libc::c_schar;
            }
            if (*chan).fx_count as libc::c_int == 1 as libc::c_int {
                (*chan).arpeggio_add = (param >> 4 as libc::c_int) as libc::c_schar;
            }
            if (*chan).fx_count as libc::c_int == 2 as libc::c_int {
                (*chan).arpeggio_add =
                    (param & 0xf as libc::c_int as libc::c_long) as libc::c_schar;
            }
        }
        25 => {
            if (*chan).fx_count as libc::c_long >= param {
                (*chan).fx_count = 0 as libc::c_int as libc::c_uchar;
                (*chan).sample_idx = 0 as libc::c_int as libc::c_ulong;
            }
        }
        28 => {
            if param == (*chan).fx_count as libc::c_long {
                (*chan).volume = 0 as libc::c_int as libc::c_uchar;
            }
        }
        29 => {
            if param == (*chan).fx_count as libc::c_long {
                trigger(chan);
            }
        }
        _ => {}
    }
    if effect > 0 as libc::c_int as libc::c_long {
        update_frequency(chan);
    }
}
unsafe extern "C" fn sequence_row() -> libc::c_long {
    let mut song_end: libc::c_long = 0;
    let mut chan_idx: libc::c_long = 0;
    let mut pat_offset: libc::c_long = 0;
    let mut effect: libc::c_long = 0;
    let mut param: libc::c_long = 0;
    let mut note: *mut note = 0 as *mut note;
    song_end = 0 as libc::c_int as libc::c_long;
    if next_row < 0 as libc::c_int as libc::c_long {
        break_pattern = pattern + 1 as libc::c_int as libc::c_long;
        next_row = 0 as libc::c_int as libc::c_long;
    }
    if break_pattern >= 0 as libc::c_int as libc::c_long {
        if break_pattern >= song_length {
            next_row = 0 as libc::c_int as libc::c_long;
            break_pattern = next_row;
        }
        if break_pattern <= pattern {
            song_end = 1 as libc::c_int as libc::c_long;
        }
        pattern = break_pattern;
        chan_idx = 0 as libc::c_int as libc::c_long;
        while chan_idx < num_channels {
            channels[chan_idx as usize].pl_row = 0 as libc::c_int as libc::c_uchar;
            chan_idx += 1;
        }
        break_pattern = -(1 as libc::c_int) as libc::c_long;
    }
    row = next_row;
    next_row = row + 1 as libc::c_int as libc::c_long;
    if next_row >= 64 as libc::c_int as libc::c_long {
        next_row = -(1 as libc::c_int) as libc::c_long;
    }
    pat_offset = ((*sequence.offset(pattern as isize) as libc::c_int * 64 as libc::c_int)
        as libc::c_long
        + row)
        * num_channels
        * 4 as libc::c_int as libc::c_long;
    chan_idx = 0 as libc::c_int as libc::c_long;
    while chan_idx < num_channels {
        note = &mut (*channels.as_mut_ptr().offset(chan_idx as isize)).note;
        (*note).key = ((*pattern_data.offset(pat_offset as isize) as libc::c_int
            & 0xf as libc::c_int)
            << 8 as libc::c_int) as libc::c_ushort;
        let ref mut fresh7 = (*note).key;
        *fresh7 = (*fresh7 as libc::c_int
            | *pattern_data.offset((pat_offset + 1 as libc::c_int as libc::c_long) as isize)
                as libc::c_int) as libc::c_ushort;
        (*note).instrument = (*pattern_data
            .offset((pat_offset + 2 as libc::c_int as libc::c_long) as isize)
            as libc::c_int
            >> 4 as libc::c_int) as libc::c_uchar;
        let ref mut fresh8 = (*note).instrument;
        *fresh8 = (*fresh8 as libc::c_int
            | *pattern_data.offset(pat_offset as isize) as libc::c_int & 0x10 as libc::c_int)
            as libc::c_uchar;
        effect = (*pattern_data.offset((pat_offset + 2 as libc::c_int as libc::c_long) as isize)
            as libc::c_int
            & 0xf as libc::c_int) as libc::c_long;
        param = *pattern_data.offset((pat_offset + 3 as libc::c_int as libc::c_long) as isize)
            as libc::c_long;
        pat_offset += 4 as libc::c_int as libc::c_long;
        if effect == 0xe as libc::c_int as libc::c_long {
            effect = 0x10 as libc::c_int as libc::c_long | param >> 4 as libc::c_int;
            param &= 0xf as libc::c_int as libc::c_long;
        }
        if effect == 0 as libc::c_int as libc::c_long && param > 0 as libc::c_int as libc::c_long {
            effect = 0xe as libc::c_int as libc::c_long;
        }
        (*note).effect = effect as libc::c_uchar;
        (*note).param = param as libc::c_uchar;
        channel_row(&mut *channels.as_mut_ptr().offset(chan_idx as isize));
        chan_idx += 1;
    }
    return song_end;
}
unsafe extern "C" fn sequence_tick() -> libc::c_long {
    let mut song_end: libc::c_long = 0;
    let mut chan_idx: libc::c_long = 0;
    song_end = 0 as libc::c_int as libc::c_long;
    tick -= 1;
    if tick <= 0 as libc::c_int as libc::c_long {
        tick = speed;
        song_end = sequence_row();
    } else {
        chan_idx = 0 as libc::c_int as libc::c_long;
        while chan_idx < num_channels {
            channel_tick(&mut *channels.as_mut_ptr().offset(chan_idx as isize));
            chan_idx += 1;
        }
    }
    return song_end;
}
unsafe extern "C" fn resample(
    mut chan: *mut channel,
    mut buf: *mut libc::c_short,
    mut offset: libc::c_long,
    mut count: libc::c_long,
) {
    let mut epos: libc::c_ulong = 0;
    let mut buf_idx: libc::c_ulong = (offset << 1 as libc::c_int) as libc::c_ulong;
    let mut buf_end: libc::c_ulong = (offset + count << 1 as libc::c_int) as libc::c_ulong;
    let mut sidx: libc::c_ulong = (*chan).sample_idx;
    let mut step: libc::c_ulong = (*chan).step;
    let mut llen: libc::c_ulong = instruments[(*chan).instrument as usize].loop_length;
    let mut lep1: libc::c_ulong =
        (instruments[(*chan).instrument as usize].loop_start).wrapping_add(llen);
    let mut sdat: *mut libc::c_schar = instruments[(*chan).instrument as usize].sample_data;
    let mut ampl: libc::c_short = (if !buf.is_null() && (*chan).mute == 0 {
        (*chan).ampl as libc::c_int
    } else {
        0 as libc::c_int
    }) as libc::c_short;
    let mut lamp: libc::c_short = (ampl as libc::c_int
        * (127 as libc::c_int - (*chan).panning as libc::c_int)
        >> 5 as libc::c_int) as libc::c_short;
    let mut ramp: libc::c_short =
        (ampl as libc::c_int * (*chan).panning as libc::c_int >> 5 as libc::c_int) as libc::c_short;
    while buf_idx < buf_end {
        if sidx >= lep1 {
            if llen <= 16384 as libc::c_int as libc::c_ulong {
                sidx = lep1;
                break;
            } else {
                while sidx >= lep1 {
                    sidx = sidx.wrapping_sub(llen);
                }
            }
        }
        epos = sidx
            .wrapping_add((buf_end.wrapping_sub(buf_idx) >> 1 as libc::c_int).wrapping_mul(step));
        if lamp as libc::c_int != 0 || ramp as libc::c_int != 0 {
            if epos > lep1 {
                epos = lep1;
            }
            if lamp as libc::c_int != 0 && ramp as libc::c_int != 0 {
                while sidx < epos {
                    ampl = *sdat.offset((sidx >> 14 as libc::c_int) as isize) as libc::c_short;
                    let fresh9 = buf_idx;
                    buf_idx = buf_idx.wrapping_add(1);
                    let ref mut fresh10 = *buf.offset(fresh9 as isize);
                    *fresh10 = (*fresh10 as libc::c_int
                        + (ampl as libc::c_int * lamp as libc::c_int >> 2 as libc::c_int))
                        as libc::c_short;
                    let fresh11 = buf_idx;
                    buf_idx = buf_idx.wrapping_add(1);
                    let ref mut fresh12 = *buf.offset(fresh11 as isize);
                    *fresh12 = (*fresh12 as libc::c_int
                        + (ampl as libc::c_int * ramp as libc::c_int >> 2 as libc::c_int))
                        as libc::c_short;
                    sidx = sidx.wrapping_add(step);
                }
            } else {
                if ramp != 0 {
                    buf_idx = buf_idx.wrapping_add(1);
                }
                while sidx < epos {
                    let ref mut fresh13 = *buf.offset(buf_idx as isize);
                    *fresh13 = (*fresh13 as libc::c_int
                        + *sdat.offset((sidx >> 14 as libc::c_int) as isize) as libc::c_int
                            * ampl as libc::c_int) as libc::c_short;
                    buf_idx = buf_idx.wrapping_add(2 as libc::c_int as libc::c_ulong);
                    sidx = sidx.wrapping_add(step);
                }
                buf_idx &= -(2 as libc::c_int) as libc::c_ulong;
            }
        } else {
            buf_idx = buf_end;
            sidx = epos;
        }
    }
    (*chan).sample_idx = sidx;
}
#[no_mangle]
pub unsafe extern "C" fn micromod_get_version() -> *const libc::c_char {
    return MICROMOD_VERSION;
}
#[no_mangle]
pub unsafe extern "C" fn micromod_calculate_mod_file_len(
    mut module_header: *mut libc::c_schar,
) -> libc::c_long {
    let mut length: libc::c_long = 0;
    let mut numchan: libc::c_long = 0;
    let mut inst_idx: libc::c_long = 0;
    numchan = calculate_num_channels(module_header);
    if numchan <= 0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int) as libc::c_long;
    }
    length = 1084 as libc::c_int as libc::c_long
        + 4 as libc::c_int as libc::c_long
            * numchan
            * 64 as libc::c_int as libc::c_long
            * calculate_num_patterns(module_header);
    inst_idx = 1 as libc::c_int as libc::c_long;
    while inst_idx < 32 as libc::c_int as libc::c_long {
        length += unsigned_short_big_endian(
            module_header,
            inst_idx * 30 as libc::c_int as libc::c_long + 12 as libc::c_int as libc::c_long,
        ) * 2 as libc::c_int as libc::c_long;
        inst_idx += 1;
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn micromod_initialise(
    mut data: *mut libc::c_schar,
    mut sampling_rate: libc::c_long,
) -> libc::c_long {
    let mut inst: *mut instrument = 0 as *mut instrument;
    let mut sample_data_offset: libc::c_long = 0;
    let mut inst_idx: libc::c_long = 0;
    let mut sample_length: libc::c_long = 0;
    let mut volume: libc::c_long = 0;
    let mut fine_tune: libc::c_long = 0;
    let mut loop_start: libc::c_long = 0;
    let mut loop_length: libc::c_long = 0;
    num_channels = calculate_num_channels(data);
    if num_channels <= 0 as libc::c_int as libc::c_long {
        num_channels = 0 as libc::c_int as libc::c_long;
        return -(1 as libc::c_int) as libc::c_long;
    }
    if sampling_rate < 8000 as libc::c_int as libc::c_long {
        return -(2 as libc::c_int) as libc::c_long;
    }
    module_data = data;
    sample_rate = sampling_rate;
    song_length = (*module_data.offset(950 as libc::c_int as isize) as libc::c_int
        & 0x7f as libc::c_int) as libc::c_long;
    restart = (*module_data.offset(951 as libc::c_int as isize) as libc::c_int
        & 0x7f as libc::c_int) as libc::c_long;
    if restart >= song_length {
        restart = 0 as libc::c_int as libc::c_long;
    }
    sequence = (module_data as *mut libc::c_uchar).offset(952 as libc::c_int as isize);
    pattern_data = (module_data as *mut libc::c_uchar).offset(1084 as libc::c_int as isize);
    num_patterns = calculate_num_patterns(module_data);
    sample_data_offset = 1084 as libc::c_int as libc::c_long
        + num_patterns
            * 64 as libc::c_int as libc::c_long
            * num_channels
            * 4 as libc::c_int as libc::c_long;
    inst_idx = 1 as libc::c_int as libc::c_long;
    while inst_idx < 32 as libc::c_int as libc::c_long {
        inst = &mut *instruments.as_mut_ptr().offset(inst_idx as isize) as *mut instrument;
        sample_length = unsigned_short_big_endian(
            module_data,
            inst_idx * 30 as libc::c_int as libc::c_long + 12 as libc::c_int as libc::c_long,
        ) * 2 as libc::c_int as libc::c_long;
        fine_tune = (*module_data.offset(
            (inst_idx * 30 as libc::c_int as libc::c_long + 14 as libc::c_int as libc::c_long)
                as isize,
        ) as libc::c_int
            & 0xf as libc::c_int) as libc::c_long;
        (*inst).fine_tune = ((fine_tune & 0x7 as libc::c_int as libc::c_long)
            - (fine_tune & 0x8 as libc::c_int as libc::c_long)
            + 8 as libc::c_int as libc::c_long) as libc::c_uchar;
        volume = (*module_data.offset(
            (inst_idx * 30 as libc::c_int as libc::c_long + 15 as libc::c_int as libc::c_long)
                as isize,
        ) as libc::c_int
            & 0x7f as libc::c_int) as libc::c_long;
        (*inst).volume = (if volume > 64 as libc::c_int as libc::c_long {
            64 as libc::c_int as libc::c_long
        } else {
            volume
        }) as libc::c_uchar;
        loop_start = unsigned_short_big_endian(
            module_data,
            inst_idx * 30 as libc::c_int as libc::c_long + 16 as libc::c_int as libc::c_long,
        ) * 2 as libc::c_int as libc::c_long;
        loop_length = unsigned_short_big_endian(
            module_data,
            inst_idx * 30 as libc::c_int as libc::c_long + 18 as libc::c_int as libc::c_long,
        ) * 2 as libc::c_int as libc::c_long;
        if loop_start + loop_length > sample_length {
            if loop_start / 2 as libc::c_int as libc::c_long + loop_length <= sample_length {
                loop_start = loop_start / 2 as libc::c_int as libc::c_long;
            } else {
                loop_length = sample_length - loop_start;
            }
        }
        if loop_length < 4 as libc::c_int as libc::c_long {
            loop_start = sample_length;
            loop_length = 0 as libc::c_int as libc::c_long;
        }
        (*inst).loop_start = (loop_start << 14 as libc::c_int) as libc::c_ulong;
        (*inst).loop_length = (loop_length << 14 as libc::c_int) as libc::c_ulong;
        let ref mut fresh14 = (*inst).sample_data;
        *fresh14 = module_data.offset(sample_data_offset as isize);
        sample_data_offset += sample_length;
        inst_idx += 1;
    }
    c2_rate = (if num_channels > 4 as libc::c_int as libc::c_long {
        8363 as libc::c_int
    } else {
        8287 as libc::c_int
    }) as libc::c_long;
    gain = (if num_channels > 4 as libc::c_int as libc::c_long {
        32 as libc::c_int
    } else {
        64 as libc::c_int
    }) as libc::c_long;
    micromod_mute_channel(-(1 as libc::c_int) as libc::c_long);
    micromod_set_position(0 as libc::c_int as libc::c_long);
    return 0 as libc::c_int as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn micromod_get_string(
    mut instrument: libc::c_long,
    mut string: *mut libc::c_char,
) {
    let mut index: libc::c_long = 0;
    let mut offset: libc::c_long = 0;
    let mut length: libc::c_long = 0;
    let mut character: libc::c_long = 0;
    if num_channels <= 0 as libc::c_int as libc::c_long {
        *string.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
        return;
    }
    offset = 0 as libc::c_int as libc::c_long;
    length = 20 as libc::c_int as libc::c_long;
    if instrument > 0 as libc::c_int as libc::c_long
        && instrument < 32 as libc::c_int as libc::c_long
    {
        offset = (instrument - 1 as libc::c_int as libc::c_long)
            * 30 as libc::c_int as libc::c_long
            + 20 as libc::c_int as libc::c_long;
        length = 22 as libc::c_int as libc::c_long;
    }
    index = 0 as libc::c_int as libc::c_long;
    while index < length {
        character = *module_data.offset((offset + index) as isize) as libc::c_long;
        if character < 32 as libc::c_int as libc::c_long
            || character > 126 as libc::c_int as libc::c_long
        {
            character = ' ' as i32 as libc::c_long;
        }
        *string.offset(index as isize) = character as libc::c_char;
        index += 1;
    }
    *string.offset(length as isize) = 0 as libc::c_int as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn micromod_calculate_song_duration() -> libc::c_long {
    let mut duration: libc::c_long = 0;
    let mut song_end: libc::c_long = 0;
    duration = 0 as libc::c_int as libc::c_long;
    if num_channels > 0 as libc::c_int as libc::c_long {
        micromod_set_position(0 as libc::c_int as libc::c_long);
        song_end = 0 as libc::c_int as libc::c_long;
        while song_end == 0 {
            duration += tick_len;
            song_end = sequence_tick();
        }
        micromod_set_position(0 as libc::c_int as libc::c_long);
    }
    return duration;
}
#[no_mangle]
pub unsafe extern "C" fn micromod_set_position(mut pos: libc::c_long) {
    let mut chan_idx: libc::c_long = 0;
    let mut chan: *mut channel = 0 as *mut channel;
    if num_channels <= 0 as libc::c_int as libc::c_long {
        return;
    }
    if pos >= song_length {
        pos = 0 as libc::c_int as libc::c_long;
    }
    break_pattern = pos;
    next_row = 0 as libc::c_int as libc::c_long;
    tick = 1 as libc::c_int as libc::c_long;
    speed = 6 as libc::c_int as libc::c_long;
    set_tempo(125 as libc::c_int as libc::c_long);
    pl_channel = -(1 as libc::c_int) as libc::c_long;
    pl_count = pl_channel;
    random_seed = 0xabcdef as libc::c_int as libc::c_long;
    chan_idx = 0 as libc::c_int as libc::c_long;
    while chan_idx < num_channels {
        chan = &mut *channels.as_mut_ptr().offset(chan_idx as isize) as *mut channel;
        (*chan).id = chan_idx as libc::c_uchar;
        let ref mut fresh15 = (*chan).assigned;
        *fresh15 = 0 as libc::c_int as libc::c_uchar;
        (*chan).instrument = *fresh15;
        (*chan).volume = 0 as libc::c_int as libc::c_uchar;
        match chan_idx & 0x3 as libc::c_int as libc::c_long {
            0 | 3 => {
                (*chan).panning = 0 as libc::c_int as libc::c_uchar;
            }
            1 | 2 => {
                (*chan).panning = 127 as libc::c_int as libc::c_uchar;
            }
            _ => {}
        }
        chan_idx += 1;
    }
    sequence_tick();
    tick_offset = 0 as libc::c_int as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn micromod_mute_channel(mut channel: libc::c_long) -> libc::c_long {
    let mut chan_idx: libc::c_long = 0;
    if channel < 0 as libc::c_int as libc::c_long {
        chan_idx = 0 as libc::c_int as libc::c_long;
        while chan_idx < num_channels {
            channels[chan_idx as usize].mute = 0 as libc::c_int as libc::c_uchar;
            chan_idx += 1;
        }
    } else if channel < num_channels {
        channels[channel as usize].mute = 1 as libc::c_int as libc::c_uchar;
    }
    return num_channels;
}
#[no_mangle]
pub unsafe extern "C" fn micromod_set_gain(mut value: libc::c_long) {
    gain = value;
}
#[no_mangle]
pub unsafe extern "C" fn micromod_get_audio(
    mut output_buffer: *mut libc::c_short,
    mut count: libc::c_long,
) {
    let mut offset: libc::c_long = 0;
    let mut remain: libc::c_long = 0;
    let mut chan_idx: libc::c_long = 0;
    if num_channels <= 0 as libc::c_int as libc::c_long {
        return;
    }
    offset = 0 as libc::c_int as libc::c_long;
    while count > 0 as libc::c_int as libc::c_long {
        remain = tick_len - tick_offset;
        if remain > count {
            remain = count;
        }
        chan_idx = 0 as libc::c_int as libc::c_long;
        while chan_idx < num_channels {
            resample(
                &mut *channels.as_mut_ptr().offset(chan_idx as isize),
                output_buffer,
                offset,
                remain,
            );
            chan_idx += 1;
        }
        tick_offset += remain;
        if tick_offset == tick_len {
            sequence_tick();
            tick_offset = 0 as libc::c_int as libc::c_long;
        }
        offset += remain;
        count -= remain;
    }
}

use std::io::{BufWriter, Write as _};

fn main() {
    let mut mod_data = std::fs::read(std::env::args_os().nth(1).unwrap()).unwrap();
    let output_file = std::fs::File::create("output.pcm").unwrap();
    let mut writer = BufWriter::new(output_file);
    unsafe {
        dbg!(micromod_initialise(
            mod_data.as_mut_ptr() as *mut i8,
            48_000
        ));
        for _ in 0..1000 {
            let mut out = [0; 4096];
            micromod_get_audio(out.as_mut_ptr(), 2048);
            for sample in out {
                writer.write_all(sample.to_le_bytes().as_slice()).unwrap();
            }
        }
    }
}

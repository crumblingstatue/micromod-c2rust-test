#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct Channel {
    pub note: Note,
    pub period: u16,
    pub porta_period: u16,
    pub sample_offset: u64,
    pub sample_idx: u64,
    pub step: u64,
    pub volume: u8,
    pub panning: u8,
    pub fine_tune: u8,
    pub ampl: u8,
    pub mute: u8,
    pub id: u8,
    pub instrument: u8,
    pub assigned: u8,
    pub porta_speed: u8,
    pub pl_row: u8,
    pub fx_count: u8,
    pub vibrato_type: u8,
    pub vibrato_phase: u8,
    pub vibrato_speed: u8,
    pub vibrato_depth: u8,
    pub tremolo_type: u8,
    pub tremolo_phase: u8,
    pub tremolo_speed: u8,
    pub tremolo_depth: u8,
    pub tremolo_add: i8,
    pub vibrato_add: i8,
    pub arpeggio_add: i8,
}
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct Note {
    pub key: u16,
    pub instrument: u8,
    pub effect: u8,
    pub param: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Instrument {
    pub volume: u8,
    pub fine_tune: u8,
    pub loop_start: u64,
    pub loop_length: u64,
    pub sample_data: *const i8,
}

impl Default for Instrument {
    fn default() -> Self {
        Self {
            volume: Default::default(),
            fine_tune: Default::default(),
            loop_start: Default::default(),
            loop_length: Default::default(),
            sample_data: std::ptr::null_mut(),
        }
    }
}
static mut MICROMOD_VERSION: *const i8 =
    b"Micromod Protracker replay 20180625 (c)mumart@gmail.com\0" as *const u8 as *const i8;
static mut FINE_TUNING: [u16; 16] = [
    4340 as i32 as u16,
    4308 as i32 as u16,
    4277 as i32 as u16,
    4247 as i32 as u16,
    4216 as i32 as u16,
    4186 as i32 as u16,
    4156 as i32 as u16,
    4126 as i32 as u16,
    4096 as i32 as u16,
    4067 as i32 as u16,
    4037 as i32 as u16,
    4008 as i32 as u16,
    3979 as i32 as u16,
    3951 as i32 as u16,
    3922 as i32 as u16,
    3894 as i32 as u16,
];
static mut ARP_TUNING: [u16; 16] = [
    4096 as i32 as u16,
    3866 as i32 as u16,
    3649 as i32 as u16,
    3444 as i32 as u16,
    3251 as i32 as u16,
    3069 as i32 as u16,
    2896 as i32 as u16,
    2734 as i32 as u16,
    2580 as i32 as u16,
    2435 as i32 as u16,
    2299 as i32 as u16,
    2170 as i32 as u16,
    2048 as i32 as u16,
    1933 as i32 as u16,
    1825 as i32 as u16,
    1722 as i32 as u16,
];
static mut SINE_TABLE: [u8; 32] = [
    0 as i32 as u8,
    24 as i32 as u8,
    49 as i32 as u8,
    74 as i32 as u8,
    97 as i32 as u8,
    120 as i32 as u8,
    141 as i32 as u8,
    161 as i32 as u8,
    180 as i32 as u8,
    197 as i32 as u8,
    212 as i32 as u8,
    224 as i32 as u8,
    235 as i32 as u8,
    244 as i32 as u8,
    250 as i32 as u8,
    253 as i32 as u8,
    255 as i32 as u8,
    253 as i32 as u8,
    250 as i32 as u8,
    244 as i32 as u8,
    235 as i32 as u8,
    224 as i32 as u8,
    212 as i32 as u8,
    197 as i32 as u8,
    180 as i32 as u8,
    161 as i32 as u8,
    141 as i32 as u8,
    120 as i32 as u8,
    97 as i32 as u8,
    74 as i32 as u8,
    49 as i32 as u8,
    24 as i32 as u8,
];

pub struct State {
    sample_rate: i64,
    gain: i64,
    c2_rate: i64,
    tick_len: i64,
    tick_offset: i64,
    pattern: i64,
    break_pattern: i64,
    row: i64,
    next_row: i64,
    tick: i64,
    speed: i64,
    pl_count: i64,
    pl_channel: i64,
    random_seed: i64,
    channels: [Channel; 16],
    instruments: [Instrument; 32],
    module_data: *const i8,
    pattern_data: *const u8,
    sequence: *const u8,
    song_length: i64,
    restart: i64,
    num_patterns: i64,
    num_channels: i64,
}

impl Default for State {
    fn default() -> Self {
        Self {
            sample_rate: Default::default(),
            gain: Default::default(),
            c2_rate: Default::default(),
            tick_len: Default::default(),
            tick_offset: Default::default(),
            pattern: Default::default(),
            break_pattern: Default::default(),
            row: Default::default(),
            next_row: Default::default(),
            tick: Default::default(),
            speed: Default::default(),
            pl_count: Default::default(),
            pl_channel: Default::default(),
            random_seed: Default::default(),
            channels: Default::default(),
            instruments: Default::default(),
            module_data: std::ptr::null_mut(),
            pattern_data: std::ptr::null_mut(),
            sequence: std::ptr::null_mut(),
            song_length: Default::default(),
            restart: Default::default(),
            num_patterns: Default::default(),
            num_channels: Default::default(),
        }
    }
}

unsafe fn calculate_num_patterns(module_header: *const i8) -> i64 {
    let mut num_patterns_0;
    let mut order_entry;
    let mut pattern_0;
    num_patterns_0 = 0 as i32 as i64;
    pattern_0 = 0 as i32 as i64;
    while pattern_0 < 128 as i32 as i64 {
        order_entry = (*module_header.offset((952 as i32 as i64 + pattern_0) as isize) as i32
            & 0x7f as i32) as i64;
        if order_entry >= num_patterns_0 {
            num_patterns_0 = order_entry + 1 as i32 as i64;
        }
        pattern_0 += 1;
    }
    return num_patterns_0;
}
unsafe fn calculate_num_channels(module_header: *const i8) -> i64 {
    let mut numchan: i64 = 0;
    let mut current_block_3: u64;
    match (*module_header.offset(1082 as i32 as isize) as i32) << 8 as i32
        | *module_header.offset(1083 as i32 as isize) as i32
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
            numchan = (*module_header.offset(1080 as i32 as isize) as i32 - 48 as i32) as i64;
            current_block_3 = 3276175668257526147;
        }
        17224 => {
            numchan = ((*module_header.offset(1080 as i32 as isize) as i32 - 48 as i32) * 10 as i32
                + (*module_header.offset(1081 as i32 as isize) as i32 - 48 as i32))
                as i64;
            current_block_3 = 3276175668257526147;
        }
        _ => {
            numchan = 0 as i32 as i64;
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
            numchan = 4 as i32 as i64;
        }
        _ => {}
    }
    if numchan > 16 as i32 as i64 {
        numchan = 0 as i32 as i64;
    }
    return numchan;
}
unsafe fn unsigned_short_big_endian(buf: *const i8, offset: i64) -> i64 {
    return ((*buf.offset(offset as isize) as i32 & 0xff as i32) << 8 as i32
        | *buf.offset((offset + 1 as i32 as i64) as isize) as i32 & 0xff as i32) as i64;
}
unsafe fn set_tempo(tempo: i64, state: &mut State) {
    state.tick_len = ((state.sample_rate << 1 as i32) + (state.sample_rate >> 1 as i32)) / tempo;
}
unsafe fn update_frequency(chan: *mut Channel, state: &mut State) {
    let mut period;
    let mut volume;
    let freq;
    period = ((*chan).period as i32 + (*chan).vibrato_add as i32) as i64;
    period = period * ARP_TUNING[(*chan).arpeggio_add as usize] as i64 >> 11 as i32;
    period = (period >> 1 as i32) + (period & 1 as i32 as i64);
    if period < 14 as i32 as i64 {
        period = 6848 as i32 as i64;
    }
    freq = (state.c2_rate * 428 as i32 as i64 / period) as u64;
    (*chan).step = (freq << 14 as i32).wrapping_div(state.sample_rate as u64);
    volume = ((*chan).volume as i32 + (*chan).tremolo_add as i32) as i64;
    if volume > 64 as i32 as i64 {
        volume = 64 as i32 as i64;
    }
    if volume < 0 as i32 as i64 {
        volume = 0 as i32 as i64;
    }
    (*chan).ampl = (volume * state.gain >> 5 as i32) as u8;
}
unsafe fn tone_portamento(chan: *mut Channel) {
    let mut source;
    let dest;
    source = (*chan).period as i64;
    dest = (*chan).porta_period as i64;
    if source < dest {
        source += (*chan).porta_speed as i64;
        if source > dest {
            source = dest;
        }
    } else if source > dest {
        source -= (*chan).porta_speed as i64;
        if source < dest {
            source = dest;
        }
    }
    (*chan).period = source as u16;
}
unsafe fn volume_slide(chan: *mut Channel, param: i64) {
    let mut volume;
    volume = (*chan).volume as i64 + (param >> 4 as i32) - (param & 0xf as i32 as i64);
    if volume < 0 as i32 as i64 {
        volume = 0 as i32 as i64;
    }
    if volume > 64 as i32 as i64 {
        volume = 64 as i32 as i64;
    }
    (*chan).volume = volume as u8;
}
unsafe fn waveform(phase: i64, type_0: i64, state: &mut State) -> i64 {
    let mut amplitude: i64 = 0 as i32 as i64;
    match type_0 & 0x3 as i32 as i64 {
        0 => {
            amplitude = SINE_TABLE[(phase & 0x1f as i32 as i64) as usize] as i64;
            if phase & 0x20 as i32 as i64 > 0 as i32 as i64 {
                amplitude = -amplitude;
            }
        }
        1 => {
            amplitude =
                255 as i32 as i64 - ((phase + 0x20 as i32 as i64 & 0x3f as i32 as i64) << 3 as i32);
        }
        2 => {
            amplitude = 255 as i32 as i64 - ((phase & 0x20 as i32 as i64) << 4 as i32);
        }
        3 => {
            amplitude = (state.random_seed >> 20 as i32) - 255 as i32 as i64;
            state.random_seed =
                state.random_seed * 65 as i32 as i64 + 17 as i32 as i64 & 0x1fffffff as i32 as i64;
        }
        _ => {}
    }
    return amplitude;
}
unsafe fn vibrato(chan: *mut Channel, state: &mut State) {
    (*chan).vibrato_add = (waveform(
        (*chan).vibrato_phase as i64,
        (*chan).vibrato_type as i64,
        state,
    ) * (*chan).vibrato_depth as i64
        >> 7 as i32) as i8;
}
unsafe fn tremolo(chan: *mut Channel, state: &mut State) {
    (*chan).tremolo_add = (waveform(
        (*chan).tremolo_phase as i64,
        (*chan).tremolo_type as i64,
        state,
    ) * (*chan).tremolo_depth as i64
        >> 6 as i32) as i8;
}
unsafe fn trigger(channel: *mut Channel, state: &mut State) {
    let period;
    let ins;
    ins = (*channel).note.instrument as i64;
    if ins > 0 as i32 as i64 && ins < 32 as i32 as i64 {
        (*channel).assigned = ins as u8;
        (*channel).sample_offset = 0 as i32 as u64;
        (*channel).fine_tune = state.instruments[ins as usize].fine_tune;
        (*channel).volume = state.instruments[ins as usize].volume;
        if state.instruments[ins as usize].loop_length > 0 as i32 as u64
            && (*channel).instrument as i32 > 0 as i32
        {
            (*channel).instrument = ins as u8;
        }
    }
    if (*channel).note.effect as i32 == 0x9 as i32 {
        (*channel).sample_offset =
            (((*channel).note.param as i32 & 0xff as i32) << 8 as i32) as u64;
    } else if (*channel).note.effect as i32 == 0x15 as i32 {
        (*channel).fine_tune = (*channel).note.param;
    }
    if (*channel).note.key as i32 > 0 as i32 {
        period = ((*channel).note.key as i32
            * FINE_TUNING[((*channel).fine_tune as i32 & 0xf as i32) as usize] as i32
            >> 11 as i32) as i64;
        (*channel).porta_period = ((period >> 1 as i32) + (period & 1 as i32 as i64)) as u16;
        if (*channel).note.effect as i32 != 0x3 as i32
            && (*channel).note.effect as i32 != 0x5 as i32
        {
            (*channel).instrument = (*channel).assigned;
            (*channel).period = (*channel).porta_period;
            (*channel).sample_idx = (*channel).sample_offset << 14 as i32;
            if ((*channel).vibrato_type as i32) < 4 as i32 {
                (*channel).vibrato_phase = 0 as i32 as u8;
            }
            if ((*channel).tremolo_type as i32) < 4 as i32 {
                (*channel).tremolo_phase = 0 as i32 as u8;
            }
        }
    }
}
unsafe fn channel_row(chan: *mut Channel, state: &mut State) {
    let effect;
    let param;
    let volume;
    let period;
    effect = (*chan).note.effect as i64;
    param = (*chan).note.param as i64;
    let ref mut fresh0 = (*chan).fx_count;
    *fresh0 = 0 as i32 as u8;
    let ref mut fresh1 = (*chan).arpeggio_add;
    *fresh1 = *fresh0 as i8;
    let ref mut fresh2 = (*chan).tremolo_add;
    *fresh2 = *fresh1;
    (*chan).vibrato_add = *fresh2;
    if !(effect == 0x1d as i32 as i64 && param > 0 as i32 as i64) {
        trigger(chan, state);
    }
    match effect {
        3 => {
            if param > 0 as i32 as i64 {
                (*chan).porta_speed = param as u8;
            }
        }
        4 => {
            if param & 0xf0 as i32 as i64 > 0 as i32 as i64 {
                (*chan).vibrato_speed = (param >> 4 as i32) as u8;
            }
            if param & 0xf as i32 as i64 > 0 as i32 as i64 {
                (*chan).vibrato_depth = (param & 0xf as i32 as i64) as u8;
            }
            vibrato(chan, state);
        }
        6 => {
            vibrato(chan, state);
        }
        7 => {
            if param & 0xf0 as i32 as i64 > 0 as i32 as i64 {
                (*chan).tremolo_speed = (param >> 4 as i32) as u8;
            }
            if param & 0xf as i32 as i64 > 0 as i32 as i64 {
                (*chan).tremolo_depth = (param & 0xf as i32 as i64) as u8;
            }
            tremolo(chan, state);
        }
        8 => {
            if state.num_channels != 4 as i32 as i64 {
                (*chan).panning = (if param < 128 as i32 as i64 {
                    param
                } else {
                    127 as i32 as i64
                }) as u8;
            }
        }
        11 => {
            if state.pl_count < 0 as i32 as i64 {
                state.break_pattern = param;
                state.next_row = 0 as i32 as i64;
            }
        }
        12 => {
            (*chan).volume = (if param > 64 as i32 as i64 {
                64 as i32 as i64
            } else {
                param
            }) as u8;
        }
        13 => {
            if state.pl_count < 0 as i32 as i64 {
                if state.break_pattern < 0 as i32 as i64 {
                    state.break_pattern = state.pattern + 1 as i32 as i64;
                }
                state.next_row =
                    (param >> 4 as i32) * 10 as i32 as i64 + (param & 0xf as i32 as i64);
                if state.next_row >= 64 as i32 as i64 {
                    state.next_row = 0 as i32 as i64;
                }
            }
        }
        15 => {
            if param > 0 as i32 as i64 {
                if param < 32 as i32 as i64 {
                    state.speed = param;
                    state.tick = state.speed;
                } else {
                    set_tempo(param, state);
                }
            }
        }
        17 => {
            period = (*chan).period as i64 - param;
            (*chan).period = (if period < 0 as i32 as i64 {
                0 as i32 as i64
            } else {
                period
            }) as u16;
        }
        18 => {
            period = (*chan).period as i64 + param;
            (*chan).period = (if period > 65535 as i32 as i64 {
                65535 as i32 as i64
            } else {
                period
            }) as u16;
        }
        20 => {
            if param < 8 as i32 as i64 {
                (*chan).vibrato_type = param as u8;
            }
        }
        22 => {
            if param == 0 as i32 as i64 {
                (*chan).pl_row = state.row as u8;
            }
            if ((*chan).pl_row as i64) < state.row && state.break_pattern < 0 as i32 as i64 {
                if state.pl_count < 0 as i32 as i64 {
                    state.pl_count = param;
                    state.pl_channel = (*chan).id as i64;
                }
                if state.pl_channel == (*chan).id as i64 {
                    if state.pl_count == 0 as i32 as i64 {
                        (*chan).pl_row = (state.row + 1 as i32 as i64) as u8;
                    } else {
                        state.next_row = (*chan).pl_row as i64;
                    }
                    state.pl_count -= 1;
                }
            }
        }
        23 => {
            if param < 8 as i32 as i64 {
                (*chan).tremolo_type = param as u8;
            }
        }
        26 => {
            volume = (*chan).volume as i64 + param;
            (*chan).volume = (if volume > 64 as i32 as i64 {
                64 as i32 as i64
            } else {
                volume
            }) as u8;
        }
        27 => {
            volume = (*chan).volume as i64 - param;
            (*chan).volume = (if volume < 0 as i32 as i64 {
                0 as i32 as i64
            } else {
                volume
            }) as u8;
        }
        28 => {
            if param <= 0 as i32 as i64 {
                (*chan).volume = 0 as i32 as u8;
            }
        }
        30 => {
            state.tick = state.speed + state.speed * param;
        }
        _ => {}
    }
    update_frequency(chan, state);
}
unsafe fn channel_tick(chan: *mut Channel, state: &mut State) {
    let effect;
    let param;
    let period;
    effect = (*chan).note.effect as i64;
    param = (*chan).note.param as i64;
    let ref mut fresh3 = (*chan).fx_count;
    *fresh3 = (*fresh3).wrapping_add(1);
    match effect {
        1 => {
            period = (*chan).period as i64 - param;
            (*chan).period = (if period < 0 as i32 as i64 {
                0 as i32 as i64
            } else {
                period
            }) as u16;
        }
        2 => {
            period = (*chan).period as i64 + param;
            (*chan).period = (if period > 65535 as i32 as i64 {
                65535 as i32 as i64
            } else {
                period
            }) as u16;
        }
        3 => {
            tone_portamento(chan);
        }
        4 => {
            let ref mut fresh4 = (*chan).vibrato_phase;
            *fresh4 = (*fresh4 as i32 + (*chan).vibrato_speed as i32) as u8;
            vibrato(chan, state);
        }
        5 => {
            tone_portamento(chan);
            volume_slide(chan, param);
        }
        6 => {
            let ref mut fresh5 = (*chan).vibrato_phase;
            *fresh5 = (*fresh5 as i32 + (*chan).vibrato_speed as i32) as u8;
            vibrato(chan, state);
            volume_slide(chan, param);
        }
        7 => {
            let ref mut fresh6 = (*chan).tremolo_phase;
            *fresh6 = (*fresh6 as i32 + (*chan).tremolo_speed as i32) as u8;
            tremolo(chan, state);
        }
        10 => {
            volume_slide(chan, param);
        }
        14 => {
            if (*chan).fx_count as i32 > 2 as i32 {
                (*chan).fx_count = 0 as i32 as u8;
            }
            if (*chan).fx_count as i32 == 0 as i32 {
                (*chan).arpeggio_add = 0 as i32 as i8;
            }
            if (*chan).fx_count as i32 == 1 as i32 {
                (*chan).arpeggio_add = (param >> 4 as i32) as i8;
            }
            if (*chan).fx_count as i32 == 2 as i32 {
                (*chan).arpeggio_add = (param & 0xf as i32 as i64) as i8;
            }
        }
        25 => {
            if (*chan).fx_count as i64 >= param {
                (*chan).fx_count = 0 as i32 as u8;
                (*chan).sample_idx = 0 as i32 as u64;
            }
        }
        28 => {
            if param == (*chan).fx_count as i64 {
                (*chan).volume = 0 as i32 as u8;
            }
        }
        29 => {
            if param == (*chan).fx_count as i64 {
                trigger(chan, state);
            }
        }
        _ => {}
    }
    if effect > 0 as i32 as i64 {
        update_frequency(chan, state);
    }
}
unsafe fn sequence_row(state: &mut State) -> i64 {
    let mut song_end;
    let mut chan_idx;
    let mut pat_offset;
    let mut effect;
    let mut param;
    let mut note;
    song_end = 0 as i32 as i64;
    if state.next_row < 0 as i32 as i64 {
        state.break_pattern = state.pattern + 1 as i32 as i64;
        state.next_row = 0 as i32 as i64;
    }
    if state.break_pattern >= 0 as i32 as i64 {
        if state.break_pattern >= state.song_length {
            state.next_row = 0 as i32 as i64;
            state.break_pattern = state.next_row;
        }
        if state.break_pattern <= state.pattern {
            song_end = 1 as i32 as i64;
        }
        state.pattern = state.break_pattern;
        chan_idx = 0 as i32 as i64;
        while chan_idx < state.num_channels {
            state.channels[chan_idx as usize].pl_row = 0 as i32 as u8;
            chan_idx += 1;
        }
        state.break_pattern = -(1 as i32) as i64;
    }
    state.row = state.next_row;
    state.next_row = state.row + 1 as i32 as i64;
    if state.next_row >= 64 as i32 as i64 {
        state.next_row = -(1 as i32) as i64;
    }
    pat_offset = ((*state.sequence.offset(state.pattern as isize) as i32 * 64 as i32) as i64
        + state.row)
        * state.num_channels
        * 4 as i32 as i64;
    chan_idx = 0 as i32 as i64;
    while chan_idx < state.num_channels {
        note = &mut (*state.channels.as_mut_ptr().offset(chan_idx as isize)).note;
        (*note).key = ((*state.pattern_data.offset(pat_offset as isize) as i32 & 0xf as i32)
            << 8 as i32) as u16;
        let ref mut fresh7 = (*note).key;
        *fresh7 = (*fresh7 as i32
            | *state
                .pattern_data
                .offset((pat_offset + 1 as i32 as i64) as isize) as i32) as u16;
        (*note).instrument = (*state
            .pattern_data
            .offset((pat_offset + 2 as i32 as i64) as isize) as i32
            >> 4 as i32) as u8;
        let ref mut fresh8 = (*note).instrument;
        *fresh8 = (*fresh8 as i32
            | *state.pattern_data.offset(pat_offset as isize) as i32 & 0x10 as i32)
            as u8;
        effect = (*state
            .pattern_data
            .offset((pat_offset + 2 as i32 as i64) as isize) as i32
            & 0xf as i32) as i64;
        param = *state
            .pattern_data
            .offset((pat_offset + 3 as i32 as i64) as isize) as i64;
        pat_offset += 4 as i32 as i64;
        if effect == 0xe as i32 as i64 {
            effect = 0x10 as i32 as i64 | param >> 4 as i32;
            param &= 0xf as i32 as i64;
        }
        if effect == 0 as i32 as i64 && param > 0 as i32 as i64 {
            effect = 0xe as i32 as i64;
        }
        (*note).effect = effect as u8;
        (*note).param = param as u8;
        channel_row(
            &mut *state.channels.as_mut_ptr().offset(chan_idx as isize),
            state,
        );
        chan_idx += 1;
    }
    return song_end;
}
unsafe fn sequence_tick(state: &mut State) -> i64 {
    let mut song_end;
    let mut chan_idx;
    song_end = 0 as i32 as i64;
    state.tick -= 1;
    if state.tick <= 0 as i32 as i64 {
        state.tick = state.speed;
        song_end = sequence_row(state);
    } else {
        chan_idx = 0 as i32 as i64;
        while chan_idx < state.num_channels {
            channel_tick(
                &mut *state.channels.as_mut_ptr().offset(chan_idx as isize),
                state,
            );
            chan_idx += 1;
        }
    }
    return song_end;
}
unsafe fn resample(chan: *mut Channel, buf: *mut i16, offset: i64, count: i64, state: &mut State) {
    let mut epos;
    let mut buf_idx: u64 = (offset << 1 as i32) as u64;
    let buf_end: u64 = (offset + count << 1 as i32) as u64;
    let mut sidx: u64 = (*chan).sample_idx;
    let step: u64 = (*chan).step;
    let llen: u64 = state.instruments[(*chan).instrument as usize].loop_length;
    let lep1: u64 = (state.instruments[(*chan).instrument as usize].loop_start).wrapping_add(llen);
    let sdat: *const i8 = state.instruments[(*chan).instrument as usize].sample_data;
    let mut ampl: i16 = (if !buf.is_null() && (*chan).mute == 0 {
        (*chan).ampl as i32
    } else {
        0 as i32
    }) as i16;
    let lamp: i16 = (ampl as i32 * (127 as i32 - (*chan).panning as i32) >> 5 as i32) as i16;
    let ramp: i16 = (ampl as i32 * (*chan).panning as i32 >> 5 as i32) as i16;
    while buf_idx < buf_end {
        if sidx >= lep1 {
            if llen <= 16384 as i32 as u64 {
                sidx = lep1;
                break;
            } else {
                while sidx >= lep1 {
                    sidx = sidx.wrapping_sub(llen);
                }
            }
        }
        epos = sidx.wrapping_add((buf_end.wrapping_sub(buf_idx) >> 1 as i32).wrapping_mul(step));
        if lamp as i32 != 0 || ramp as i32 != 0 {
            if epos > lep1 {
                epos = lep1;
            }
            if lamp as i32 != 0 && ramp as i32 != 0 {
                while sidx < epos {
                    ampl = *sdat.offset((sidx >> 14 as i32) as isize) as i16;
                    let fresh9 = buf_idx;
                    buf_idx = buf_idx.wrapping_add(1);
                    let ref mut fresh10 = *buf.offset(fresh9 as isize);
                    *fresh10 = (*fresh10 as i32 + (ampl as i32 * lamp as i32 >> 2 as i32)) as i16;
                    let fresh11 = buf_idx;
                    buf_idx = buf_idx.wrapping_add(1);
                    let ref mut fresh12 = *buf.offset(fresh11 as isize);
                    *fresh12 = (*fresh12 as i32 + (ampl as i32 * ramp as i32 >> 2 as i32)) as i16;
                    sidx = sidx.wrapping_add(step);
                }
            } else {
                if ramp != 0 {
                    buf_idx = buf_idx.wrapping_add(1);
                }
                while sidx < epos {
                    let ref mut fresh13 = *buf.offset(buf_idx as isize);
                    *fresh13 = (*fresh13 as i32
                        + *sdat.offset((sidx >> 14 as i32) as isize) as i32 * ampl as i32)
                        as i16;
                    buf_idx = buf_idx.wrapping_add(2 as i32 as u64);
                    sidx = sidx.wrapping_add(step);
                }
                buf_idx &= -(2 as i32) as u64;
            }
        } else {
            buf_idx = buf_end;
            sidx = epos;
        }
    }
    (*chan).sample_idx = sidx;
}

pub unsafe fn micromod_get_version() -> *const i8 {
    return MICROMOD_VERSION;
}

pub unsafe fn micromod_calculate_mod_file_len(module_header: *mut i8) -> i64 {
    let mut length;
    let numchan;
    let mut inst_idx;
    numchan = calculate_num_channels(module_header);
    if numchan <= 0 as i32 as i64 {
        return -(1 as i32) as i64;
    }
    length = 1084 as i32 as i64
        + 4 as i32 as i64 * numchan * 64 as i32 as i64 * calculate_num_patterns(module_header);
    inst_idx = 1 as i32 as i64;
    while inst_idx < 32 as i32 as i64 {
        length += unsigned_short_big_endian(
            module_header,
            inst_idx * 30 as i32 as i64 + 12 as i32 as i64,
        ) * 2 as i32 as i64;
        inst_idx += 1;
    }
    return length;
}

pub unsafe fn micromod_initialise(data: &[u8], sampling_rate: i64, state: &mut State) -> i64 {
    let mut inst;
    let mut sample_data_offset;
    let mut inst_idx;
    let mut sample_length;
    let mut volume;
    let mut fine_tune;
    let mut loop_start;
    let mut loop_length;
    state.num_channels = calculate_num_channels(data.as_ptr().cast());
    if state.num_channels <= 0 as i32 as i64 {
        state.num_channels = 0 as i32 as i64;
        return -(1 as i32) as i64;
    }
    if sampling_rate < 8000 as i32 as i64 {
        return -(2 as i32) as i64;
    }
    state.module_data = data.as_ptr().cast();
    state.sample_rate = sampling_rate;
    state.song_length =
        (*state.module_data.offset(950 as i32 as isize) as i32 & 0x7f as i32) as i64;
    state.restart = (*state.module_data.offset(951 as i32 as isize) as i32 & 0x7f as i32) as i64;
    if state.restart >= state.song_length {
        state.restart = 0 as i32 as i64;
    }
    state.sequence = (state.module_data as *mut u8).offset(952 as i32 as isize);
    state.pattern_data = (state.module_data as *mut u8).offset(1084 as i32 as isize);
    state.num_patterns = calculate_num_patterns(state.module_data);
    sample_data_offset = 1084 as i32 as i64
        + state.num_patterns * 64 as i32 as i64 * state.num_channels * 4 as i32 as i64;
    inst_idx = 1 as i32 as i64;
    while inst_idx < 32 as i32 as i64 {
        inst = &mut *state.instruments.as_mut_ptr().offset(inst_idx as isize) as *mut Instrument;
        sample_length = unsigned_short_big_endian(
            state.module_data,
            inst_idx * 30 as i32 as i64 + 12 as i32 as i64,
        ) * 2 as i32 as i64;
        fine_tune = (*state
            .module_data
            .offset((inst_idx * 30 as i32 as i64 + 14 as i32 as i64) as isize)
            as i32
            & 0xf as i32) as i64;
        (*inst).fine_tune = ((fine_tune & 0x7 as i32 as i64) - (fine_tune & 0x8 as i32 as i64)
            + 8 as i32 as i64) as u8;
        volume = (*state
            .module_data
            .offset((inst_idx * 30 as i32 as i64 + 15 as i32 as i64) as isize)
            as i32
            & 0x7f as i32) as i64;
        (*inst).volume = (if volume > 64 as i32 as i64 {
            64 as i32 as i64
        } else {
            volume
        }) as u8;
        loop_start = unsigned_short_big_endian(
            state.module_data,
            inst_idx * 30 as i32 as i64 + 16 as i32 as i64,
        ) * 2 as i32 as i64;
        loop_length = unsigned_short_big_endian(
            state.module_data,
            inst_idx * 30 as i32 as i64 + 18 as i32 as i64,
        ) * 2 as i32 as i64;
        if loop_start + loop_length > sample_length {
            if loop_start / 2 as i32 as i64 + loop_length <= sample_length {
                loop_start = loop_start / 2 as i32 as i64;
            } else {
                loop_length = sample_length - loop_start;
            }
        }
        if loop_length < 4 as i32 as i64 {
            loop_start = sample_length;
            loop_length = 0 as i32 as i64;
        }
        (*inst).loop_start = (loop_start << 14 as i32) as u64;
        (*inst).loop_length = (loop_length << 14 as i32) as u64;
        let ref mut fresh14 = (*inst).sample_data;
        *fresh14 = state.module_data.offset(sample_data_offset as isize);
        sample_data_offset += sample_length;
        inst_idx += 1;
    }
    state.c2_rate = (if state.num_channels > 4 as i32 as i64 {
        8363 as i32
    } else {
        8287 as i32
    }) as i64;
    state.gain = (if state.num_channels > 4 as i32 as i64 {
        32 as i32
    } else {
        64 as i32
    }) as i64;
    micromod_mute_channel(-(1 as i32) as i64, state);
    micromod_set_position(0 as i32 as i64, state);
    return 0 as i32 as i64;
}

pub unsafe fn micromod_get_string(instrument: i64, string: *mut i8, state: &mut State) {
    let mut index;
    let mut offset;
    let mut length;
    let mut character;
    if state.num_channels <= 0 as i32 as i64 {
        *string.offset(0 as i32 as isize) = 0 as i32 as i8;
        return;
    }
    offset = 0 as i32 as i64;
    length = 20 as i32 as i64;
    if instrument > 0 as i32 as i64 && instrument < 32 as i32 as i64 {
        offset = (instrument - 1 as i32 as i64) * 30 as i32 as i64 + 20 as i32 as i64;
        length = 22 as i32 as i64;
    }
    index = 0 as i32 as i64;
    while index < length {
        character = *state.module_data.offset((offset + index) as isize) as i64;
        if character < 32 as i32 as i64 || character > 126 as i32 as i64 {
            character = ' ' as i32 as i64;
        }
        *string.offset(index as isize) = character as i8;
        index += 1;
    }
    *string.offset(length as isize) = 0 as i32 as i8;
}

pub unsafe fn micromod_calculate_song_duration(state: &mut State) -> i64 {
    let mut duration;
    let mut song_end;
    duration = 0 as i32 as i64;
    if state.num_channels > 0 as i32 as i64 {
        micromod_set_position(0 as i32 as i64, state);
        song_end = 0 as i32 as i64;
        while song_end == 0 {
            duration += state.tick_len;
            song_end = sequence_tick(state);
        }
        micromod_set_position(0 as i32 as i64, state);
    }
    return duration;
}

pub unsafe fn micromod_set_position(mut pos: i64, state: &mut State) {
    let mut chan_idx;
    let mut chan;
    if state.num_channels <= 0 as i32 as i64 {
        return;
    }
    if pos >= state.song_length {
        pos = 0 as i32 as i64;
    }
    state.break_pattern = pos;
    state.next_row = 0 as i32 as i64;
    state.tick = 1 as i32 as i64;
    state.speed = 6 as i32 as i64;
    set_tempo(125 as i32 as i64, state);
    state.pl_channel = -(1 as i32) as i64;
    state.pl_count = state.pl_channel;
    state.random_seed = 0xabcdef as i32 as i64;
    chan_idx = 0 as i32 as i64;
    while chan_idx < state.num_channels {
        chan = &mut *state.channels.as_mut_ptr().offset(chan_idx as isize) as *mut Channel;
        (*chan).id = chan_idx as u8;
        let ref mut fresh15 = (*chan).assigned;
        *fresh15 = 0 as i32 as u8;
        (*chan).instrument = *fresh15;
        (*chan).volume = 0 as i32 as u8;
        match chan_idx & 0x3 as i32 as i64 {
            0 | 3 => {
                (*chan).panning = 0 as i32 as u8;
            }
            1 | 2 => {
                (*chan).panning = 127 as i32 as u8;
            }
            _ => {}
        }
        chan_idx += 1;
    }
    sequence_tick(state);
    state.tick_offset = 0 as i32 as i64;
}

pub unsafe fn micromod_mute_channel(channel: i64, state: &mut State) -> i64 {
    let mut chan_idx;
    if channel < 0 as i32 as i64 {
        chan_idx = 0 as i32 as i64;
        while chan_idx < state.num_channels {
            state.channels[chan_idx as usize].mute = 0 as i32 as u8;
            chan_idx += 1;
        }
    } else if channel < state.num_channels {
        state.channels[channel as usize].mute = 1 as i32 as u8;
    }
    return state.num_channels;
}

pub unsafe fn micromod_set_gain(value: i64, state: &mut State) {
    state.gain = value;
}

pub unsafe fn micromod_get_audio(output_buffer: *mut i16, mut count: i64, state: &mut State) {
    let mut offset;
    let mut remain;
    let mut chan_idx;
    if state.num_channels <= 0 as i32 as i64 {
        return;
    }
    offset = 0 as i32 as i64;
    while count > 0 as i32 as i64 {
        remain = state.tick_len - state.tick_offset;
        if remain > count {
            remain = count;
        }
        chan_idx = 0 as i32 as i64;
        while chan_idx < state.num_channels {
            resample(
                &mut *state.channels.as_mut_ptr().offset(chan_idx as isize),
                output_buffer,
                offset,
                remain,
                state,
            );
            chan_idx += 1;
        }
        state.tick_offset += remain;
        if state.tick_offset == state.tick_len {
            sequence_tick(state);
            state.tick_offset = 0 as i32 as i64;
        }
        offset += remain;
        count -= remain;
    }
}

use std::io::{BufWriter, Write as _};

fn main() {
    let mod_data = std::fs::read(std::env::args_os().nth(1).unwrap()).unwrap();
    let output_file = std::fs::File::create("output.pcm").unwrap();
    let mut writer = BufWriter::new(output_file);
    let mut state = State::default();
    unsafe {
        dbg!(micromod_initialise(&mod_data, 48_000, &mut state));
        for _ in 0..1000 {
            let mut out = [0; 4096];
            micromod_get_audio(out.as_mut_ptr(), 2048, &mut state);
            for sample in out {
                writer.write_all(sample.to_le_bytes().as_slice()).unwrap();
            }
        }
    }
}

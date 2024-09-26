#[derive(Copy, Clone, Default)]
pub(crate) struct Channel {
    pub(crate) note: Note,
    pub(crate) period: u16,
    pub(crate) porta_period: u16,
    pub(crate) sample_offset: usize,
    pub(crate) sample_idx: usize,
    pub(crate) step: usize,
    pub(crate) volume: u8,
    pub(crate) panning: u8,
    pub(crate) fine_tune: u8,
    pub(crate) ampl: u8,
    pub(crate) mute: u8,
    pub(crate) id: u8,
    pub(crate) instrument: u8,
    pub(crate) assigned: u8,
    pub(crate) porta_speed: u8,
    pub(crate) pl_row: u8,
    pub(crate) fx_count: u8,
    pub(crate) vibrato_type: u8,
    pub(crate) vibrato_phase: u8,
    pub(crate) vibrato_speed: u8,
    pub(crate) vibrato_depth: u8,
    pub(crate) tremolo_type: u8,
    pub(crate) tremolo_phase: u8,
    pub(crate) tremolo_speed: u8,
    pub(crate) tremolo_depth: u8,
    pub(crate) tremolo_add: i8,
    pub(crate) vibrato_add: i8,
    pub(crate) arpeggio_add: i8,
}
#[derive(Copy, Clone, Default)]
pub(crate) struct Note {
    pub(crate) key: u16,
    pub(crate) instrument: u8,
    pub(crate) effect: u8,
    pub(crate) param: u8,
}
#[derive(Copy, Clone)]
pub(crate) struct Instrument<'src> {
    pub(crate) volume: u8,
    pub(crate) fine_tune: u8,
    pub(crate) loop_start: usize,
    pub(crate) loop_length: usize,
    pub(crate) sample_data: &'src [i8],
}

impl Instrument<'_> {
    pub(crate) const fn dummy() -> Self {
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
pub(crate) struct ModSrc<'src> {
    pub(crate) instruments: Vec<Instrument<'src>>,
    pub(crate) module_data: &'src [i8],
    pub(crate) pattern_data: &'src [u8],
    pub(crate) sequence: &'src [u8],
    pub(crate) num_patterns: i32,
    pub(crate) num_channels: i32,
    pub(crate) song_length: i32,
}

#[derive(Default)]
pub(crate) struct PlaybackState {
    pub(crate) gain: i32,
    pub(crate) c2_rate: i32,
    pub(crate) tick_len: i32,
    pub(crate) tick_offset: i32,
    pub(crate) pattern: i32,
    pub(crate) break_pattern: i32,
    pub(crate) row: i32,
    pub(crate) next_row: i32,
    pub(crate) tick: i32,
    pub(crate) speed: i32,
    pub(crate) pl_count: i32,
    pub(crate) pl_channel: i32,
    pub(crate) random_seed: i32,
}

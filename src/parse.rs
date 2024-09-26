pub(crate) fn calculate_num_patterns(module_header: &[u8]) -> u16 {
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

pub(crate) fn calculate_num_channels(module_header: &[u8]) -> Option<u16> {
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

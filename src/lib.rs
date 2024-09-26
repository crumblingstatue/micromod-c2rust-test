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
mod engine;
mod parse;
mod rendering;
mod slice_ext;
#[cfg(test)]
mod tests;
mod types;

pub use engine::Engine;

use slice_ext::ByteSliceExt as _;

/// Get a nice version string. I guess.
pub fn version() -> &'static str {
    consts::MICROMOD_VERSION
}

/// Calculate the length of the module file... In samples. Presumably.
pub fn calculate_mod_file_len(mod_data: &[u8]) -> Option<u32> {
    let numchan = u32::from(crate::parse::calculate_num_channels(mod_data)?);
    let mut length =
        1084 + 4 * numchan * 64 * u32::from(crate::parse::calculate_num_patterns(mod_data));
    let mut inst_idx = 1;
    while inst_idx < 32 {
        length += u32::from(mod_data.read_u16_be(inst_idx * 30 + 12).unwrap()) * 2;
        inst_idx += 1;
    }
    Some(length)
}

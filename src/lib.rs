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

/// Get a nice version string. I guess.
pub fn version() -> &'static str {
    consts::MICROMOD_VERSION
}

#![no_std]

mod bind;

pub mod base;

#[cfg(feature = "ntoskrnl")]
pub mod ntoskrnl;

pub use cty::*;

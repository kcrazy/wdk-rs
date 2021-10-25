#![no_std]

pub mod base;

#[cfg(feature = "ntoskrnl")]
pub mod ntoskrnl;

pub use cty::*;

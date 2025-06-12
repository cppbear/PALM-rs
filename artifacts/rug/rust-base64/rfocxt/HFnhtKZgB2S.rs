#[cfg(any(feature = "alloc", test))]
use alloc::string::String;
use core::fmt;
#[cfg(any(feature = "std", test))]
use std::error;
#[cfg(any(feature = "alloc", test))]
use crate::engine::general_purpose::STANDARD;
use crate::engine::{Config, Engine};
use crate::PAD_BYTE;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EncodeSliceError {
    /// The provided slice is too small.
    OutputSliceTooSmall,
}
impl fmt::Display for EncodeSliceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutputSliceTooSmall => write!(f, "Output slice too small"),
        }
    }
}

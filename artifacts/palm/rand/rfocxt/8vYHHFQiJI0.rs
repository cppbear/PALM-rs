use core::num::NonZeroUsize;
use crate::distr::uniform::{UniformSampler, UniformUsize};
use crate::distr::Distribution;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[derive(Debug, Clone, Copy)]
pub struct Empty;
impl core::fmt::Display for Empty {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Tried to create a `rand::distr::slice::Choose` with an empty slice")
    }
}

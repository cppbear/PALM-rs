use crate::engine::{general_purpose::STANDARD, DecodeEstimate, Engine};
#[cfg(any(feature = "alloc", test))]
use alloc::vec::Vec;
use core::fmt;
#[cfg(any(feature = "std", test))]
use std::error;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeSliceError {
    /// A [`DecodeError`] occurred
    DecodeError(DecodeError),
    /// The provided slice is too small.
    OutputSliceTooSmall,
}
#[deprecated(since = "0.21.0", note = "Use Engine::decode_slice")]
pub fn decode_engine_slice<E: Engine, T: AsRef<[u8]>>(
    input: T,
    output: &mut [u8],
    engine: &E,
) -> Result<usize, DecodeSliceError> {
    engine.decode_slice(input, output)
}

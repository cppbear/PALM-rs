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
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeError {
    /// An invalid byte was found in the input. The offset and offending byte are provided.
    ///
    /// Padding characters (`=`) interspersed in the encoded form are invalid, as they may only
    /// be present as the last 0-2 bytes of input.
    ///
    /// This error may also indicate that extraneous trailing input bytes are present, causing
    /// otherwise valid padding to no longer be the last bytes of input.
    InvalidByte(usize, u8),
    /// The length of the input, as measured in valid base64 symbols, is invalid.
    /// There must be 2-4 symbols in the last input quad.
    InvalidLength(usize),
    /// The last non-padding input symbol's encoded 6 bits have nonzero bits that will be discarded.
    /// This is indicative of corrupted or truncated Base64.
    /// Unlike [`DecodeError::InvalidByte`], which reports symbols that aren't in the alphabet,
    /// this error is for symbols that are in the alphabet but represent nonsensical encodings.
    InvalidLastSymbol(usize, u8),
    /// The nature of the padding was not as configured: absent or incorrect when it must be
    /// canonical, or present when it must be absent, etc.
    InvalidPadding,
}
impl fmt::Display for DecodeSliceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DecodeError(e) => write!(f, "DecodeError: {}", e),
            Self::OutputSliceTooSmall => write!(f, "Output slice too small"),
        }
    }
}

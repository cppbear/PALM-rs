#[cfg(any(feature = "alloc", test))]
use alloc::string::String;
use core::fmt;
#[cfg(any(feature = "std", test))]
use std::error;
#[cfg(any(feature = "alloc", test))]
use crate::engine::general_purpose::STANDARD;
use crate::engine::{Config, Engine};
use crate::PAD_BYTE;
#[derive(Debug, Clone)]
pub struct GeneralPurpose {
    encode_table: [u8; 64],
    decode_table: [u8; 256],
    config: GeneralPurposeConfig,
}
#[allow(unused)]
#[deprecated(since = "0.21.0", note = "Use Engine::encode")]
#[cfg(any(feature = "alloc", test))]
pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
    STANDARD.encode(input)
}

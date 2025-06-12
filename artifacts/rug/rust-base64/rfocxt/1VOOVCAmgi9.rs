#[cfg(any(feature = "alloc", test))]
use alloc::string::String;
use core::fmt;
#[cfg(any(feature = "std", test))]
use std::error;
#[cfg(any(feature = "alloc", test))]
use crate::engine::general_purpose::STANDARD;
use crate::engine::{Config, Engine};
use crate::PAD_BYTE;
#[allow(unused)]
#[deprecated(since = "0.21.0", note = "Use Engine::encode")]
#[cfg(any(feature = "alloc", test))]
pub fn encode_engine<E: Engine, T: AsRef<[u8]>>(input: T, engine: &E) -> String {
    engine.encode(input)
}

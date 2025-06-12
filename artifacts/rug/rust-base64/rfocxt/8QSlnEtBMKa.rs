#[cfg(any(feature = "alloc", test))]
use alloc::string::String;
use core::fmt;
#[cfg(any(feature = "std", test))]
use std::error;
#[cfg(any(feature = "alloc", test))]
use crate::engine::general_purpose::STANDARD;
use crate::engine::{Config, Engine};
use crate::PAD_BYTE;
#[must_use]
pub const fn encoded_len(bytes_len: usize, padding: bool) -> Option<usize> {
    let rem = bytes_len % 3;
    let complete_input_chunks = bytes_len / 3;
    let complete_chunk_output = if let Some(complete_chunk_output) = complete_input_chunks
        .checked_mul(4)
    {
        complete_chunk_output
    } else {
        return None;
    };
    if rem > 0 {
        if padding {
            complete_chunk_output.checked_add(4)
        } else {
            let encoded_rem = match rem {
                1 => 2,
                _ => 3,
            };
            complete_chunk_output.checked_add(encoded_rem)
        }
    } else {
        Some(complete_chunk_output)
    }
}

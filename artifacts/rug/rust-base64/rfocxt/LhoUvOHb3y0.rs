#[cfg(any(feature = "alloc", test))]
use alloc::string::String;
use core::fmt;
#[cfg(any(feature = "std", test))]
use std::error;
#[cfg(any(feature = "alloc", test))]
use crate::engine::general_purpose::STANDARD;
use crate::engine::{Config, Engine};
use crate::PAD_BYTE;
pub(crate) fn encode_with_padding<E: Engine + ?Sized>(
    input: &[u8],
    output: &mut [u8],
    engine: &E,
    expected_encoded_size: usize,
) {
    debug_assert_eq!(expected_encoded_size, output.len());
    let b64_bytes_written = engine.internal_encode(input, output);
    let padding_bytes = if engine.config().encode_padding() {
        add_padding(b64_bytes_written, &mut output[b64_bytes_written..])
    } else {
        0
    };
    let encoded_bytes = b64_bytes_written
        .checked_add(padding_bytes)
        .expect("usize overflow when calculating b64 length");
    debug_assert_eq!(expected_encoded_size, encoded_bytes);
}
pub(crate) fn add_padding(unpadded_output_len: usize, output: &mut [u8]) -> usize {
    let pad_bytes = (4 - (unpadded_output_len % 4)) % 4;
    #[allow(clippy::needless_range_loop)]
    for i in 0..pad_bytes {
        output[i] = PAD_BYTE;
    }
    pad_bytes
}

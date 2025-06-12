use crate::error::{Error, ErrorCode, Result};
use alloc::vec::Vec;
use core::cmp;
use core::mem;
use core::ops::Deref;
use core::str;
#[cfg(feature = "std")]
use crate::io;
#[cfg(feature = "std")]
use crate::iter::LineColIterator;
#[cfg(feature = "raw_value")]
use crate::raw::BorrowedRawDeserializer;
#[cfg(all(feature = "raw_value", feature = "std"))]
use crate::raw::OwnedRawDeserializer;
#[cfg(all(feature = "raw_value", feature = "std"))]
use alloc::string::String;
#[cfg(feature = "raw_value")]
use serde::de::Visitor;
static HEX0: [i16; 256] = build_hex_table(0);
static HEX1: [i16; 256] = build_hex_table(4);
const fn build_hex_table(shift: usize) -> [i16; 256] {
    let mut table = [0; 256];
    let mut ch = 0;
    while ch < 256 {
        table[ch] = match decode_hex_val_slow(ch as u8) {
            Some(val) => (val as i16) << shift,
            None => -1,
        };
        ch += 1;
    }
    table
}
const fn decode_hex_val_slow(val: u8) -> Option<u8> {
    match val {
        b'0'..=b'9' => Some(val - b'0'),
        b'A'..=b'F' => Some(val - b'A' + 10),
        b'a'..=b'f' => Some(val - b'a' + 10),
        _ => None,
    }
}

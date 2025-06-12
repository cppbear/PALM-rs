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
fn decode_four_hex_digits(a: u8, b: u8, c: u8, d: u8) -> Option<u16> {
    let a = HEX1[a as usize] as i32;
    let b = HEX0[b as usize] as i32;
    let c = HEX1[c as usize] as i32;
    let d = HEX0[d as usize] as i32;
    let codepoint = ((a | b) << 8) | c | d;
    if codepoint >= 0 { Some(codepoint as u16) } else { None }
}

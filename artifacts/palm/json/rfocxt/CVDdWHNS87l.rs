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
pub trait Read<'de>: private::Sealed {
    const should_early_return_if_failed: bool;
    fn next(&mut self) -> Result<Option<u8>>;
    fn peek(&mut self) -> Result<Option<u8>>;
    fn discard(&mut self);
    fn position(&self) -> Position;
    fn peek_position(&self) -> Position;
    fn byte_offset(&self) -> usize;
    fn parse_str<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, str>>;
    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>>;
    fn ignore_str(&mut self) -> Result<()>;
    fn decode_hex_escape(&mut self) -> Result<u16>;
    #[cfg(feature = "raw_value")]
    fn begin_raw_buffering(&mut self);
    #[cfg(feature = "raw_value")]
    fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>;
    fn set_failed(&mut self, failed: &mut bool);
}
pub trait Sealed {}
pub struct SliceRead<'a> {
    slice: &'a [u8],
    /// Index of the *next* byte that will be returned by next() or peek().
    index: usize,
    #[cfg(feature = "raw_value")]
    raw_buffering_start_index: usize,
}
pub struct Position {
    pub line: usize,
    pub column: usize,
}
impl<'a> Read<'a> for SliceRead<'a> {
    const should_early_return_if_failed: bool = false;
    #[inline]
    fn next(&mut self) -> Result<Option<u8>> {}
    #[inline]
    fn peek(&mut self) -> Result<Option<u8>> {}
    #[inline]
    fn discard(&mut self) {}
    fn position(&self) -> Position {
        self.position_of_index(self.index)
    }
    fn peek_position(&self) -> Position {}
    fn byte_offset(&self) -> usize {}
    fn parse_str<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'a, 's, str>> {}
    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'a, 's, [u8]>> {}
    fn ignore_str(&mut self) -> Result<()> {}
    #[inline]
    fn decode_hex_escape(&mut self) -> Result<u16> {}
    #[cfg(feature = "raw_value")]
    fn begin_raw_buffering(&mut self) {}
    #[cfg(feature = "raw_value")]
    fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'a>,
    {}
    #[inline]
    #[cold]
    fn set_failed(&mut self, _failed: &mut bool) {}
}
impl<'a> SliceRead<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        SliceRead {
            slice,
            index: 0,
            #[cfg(feature = "raw_value")]
            raw_buffering_start_index: 0,
        }
    }
    fn position_of_index(&self, i: usize) -> Position {
        let start_of_line = match memchr::memrchr(b'\n', &self.slice[..i]) {
            Some(position) => position + 1,
            None => 0,
        };
        Position {
            line: 1 + memchr::memchr_iter(b'\n', &self.slice[..start_of_line]).count(),
            column: i - start_of_line,
        }
    }
    fn skip_to_escape(&mut self, forbid_control_characters: bool) {}
    #[cold]
    #[inline(never)]
    fn skip_to_escape_slow(&mut self) {}
    fn parse_str_bytes<'s, T, F>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
        validate: bool,
        result: F,
    ) -> Result<Reference<'a, 's, T>>
    where
        T: ?Sized + 's,
        F: for<'f> FnOnce(&'s Self, &'f [u8]) -> Result<&'f T>,
    {}
}

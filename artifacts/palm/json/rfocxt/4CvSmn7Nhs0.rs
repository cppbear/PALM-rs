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
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub struct IoRead<R>
where
    R: io::Read,
{
    iter: LineColIterator<io::Bytes<R>>,
    /// Temporary storage of peeked byte.
    ch: Option<u8>,
    #[cfg(feature = "raw_value")]
    raw_buffer: Option<Vec<u8>>,
}
pub struct LineColIterator<I> {
    iter: I,
    /// Index of the current line. Characters in the first line of the input
    /// (before the first newline character) are in line 1.
    line: usize,
    /// Index of the current column. The first character in the input and any
    /// characters immediately following a newline character are in column 1.
    /// The column is 0 immediately after a newline character has been read.
    col: usize,
    /// Byte offset of the start of the current line. This is the sum of lengths
    /// of all previous lines. Keeping track of things this way allows efficient
    /// computation of the current line, column, and byte offset while only
    /// updating one of the counters in `next()` in the common case.
    start_of_line: usize,
}
#[cfg(feature = "std")]
impl<'de, R> Read<'de> for IoRead<R>
where
    R: io::Read,
{
    const should_early_return_if_failed: bool = true;
    #[inline]
    fn next(&mut self) -> Result<Option<u8>> {}
    #[inline]
    fn peek(&mut self) -> Result<Option<u8>> {}
    #[cfg(not(feature = "raw_value"))]
    #[inline]
    fn discard(&mut self) {}
    #[cfg(feature = "raw_value")]
    fn discard(&mut self) {}
    fn position(&self) -> Position {}
    fn peek_position(&self) -> Position {}
    fn byte_offset(&self) -> usize {
        match self.ch {
            Some(_) => self.iter.byte_offset() - 1,
            None => self.iter.byte_offset(),
        }
    }
    fn parse_str<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, str>> {}
    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>> {}
    fn ignore_str(&mut self) -> Result<()> {}
    fn decode_hex_escape(&mut self) -> Result<u16> {}
    #[cfg(feature = "raw_value")]
    fn begin_raw_buffering(&mut self) {}
    #[cfg(feature = "raw_value")]
    fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {}
    #[inline]
    #[cold]
    fn set_failed(&mut self, failed: &mut bool) {}
}

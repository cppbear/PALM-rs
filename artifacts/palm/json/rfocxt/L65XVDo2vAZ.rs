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
pub struct Error {
    /// This `Box` allows us to keep the size of `Error` as small as possible. A
    /// larger `Error` type was substantially slower due to all the functions
    /// that pass around `Result<T, Error>`.
    err: Box<ErrorImpl>,
}
pub(crate) enum ErrorCode {
    /// Catchall for syntax error messages
    Message(Box<str>),
    /// Some I/O error occurred while serializing or deserializing.
    Io(io::Error),
    /// EOF while parsing a list.
    EofWhileParsingList,
    /// EOF while parsing an object.
    EofWhileParsingObject,
    /// EOF while parsing a string.
    EofWhileParsingString,
    /// EOF while parsing a JSON value.
    EofWhileParsingValue,
    /// Expected this character to be a `':'`.
    ExpectedColon,
    /// Expected this character to be either a `','` or a `']'`.
    ExpectedListCommaOrEnd,
    /// Expected this character to be either a `','` or a `'}'`.
    ExpectedObjectCommaOrEnd,
    /// Expected to parse either a `true`, `false`, or a `null`.
    ExpectedSomeIdent,
    /// Expected this character to start a JSON value.
    ExpectedSomeValue,
    /// Expected this character to be a `"`.
    ExpectedDoubleQuote,
    /// Invalid hex escape code.
    InvalidEscape,
    /// Invalid number.
    InvalidNumber,
    /// Number is bigger than the maximum value of its type.
    NumberOutOfRange,
    /// Invalid unicode code point.
    InvalidUnicodeCodePoint,
    /// Control character found while parsing a string.
    ControlCharacterWhileParsingString,
    /// Object key is not a string.
    KeyMustBeAString,
    /// Contents of key were supposed to be a number.
    ExpectedNumericKey,
    /// Object key is a non-finite float value.
    FloatKeyMustBeFinite,
    /// Lone leading surrogate in hex escape.
    LoneLeadingSurrogateInHexEscape,
    /// JSON has a comma after the last value in an array or map.
    TrailingComma,
    /// JSON has non-whitespace trailing characters after the value.
    TrailingCharacters,
    /// Unexpected end of hex escape.
    UnexpectedEndOfHexEscape,
    /// Encountered nesting of JSON maps and arrays more than 128 layers deep.
    RecursionLimitExceeded,
}
impl<'a> Read<'a> for SliceRead<'a> {
    const should_early_return_if_failed: bool = false;
    #[inline]
    fn next(&mut self) -> Result<Option<u8>> {}
    #[inline]
    fn peek(&mut self) -> Result<Option<u8>> {}
    #[inline]
    fn discard(&mut self) {}
    fn position(&self) -> Position {}
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
    fn ignore_str(&mut self) -> Result<()> {
        loop {
            self.skip_to_escape(true);
            if self.index == self.slice.len() {
                return error(self, ErrorCode::EofWhileParsingString);
            }
            match self.slice[self.index] {
                b'"' => {
                    self.index += 1;
                    return Ok(());
                }
                b'\\' => {
                    self.index += 1;
                    tri!(ignore_escape(self));
                }
                _ => {
                    return error(self, ErrorCode::ControlCharacterWhileParsingString);
                }
            }
        }
    }
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
    fn position_of_index(&self, i: usize) -> Position {}
    fn skip_to_escape(&mut self, forbid_control_characters: bool) {
        if self.index == self.slice.len()
            || is_escape(self.slice[self.index], forbid_control_characters)
        {
            return;
        }
        self.index += 1;
        let rest = &self.slice[self.index..];
        if !forbid_control_characters {
            self.index += memchr::memchr2(b'"', b'\\', rest).unwrap_or(rest.len());
            return;
        }
        #[cfg(fast_arithmetic = "64")]
        type Chunk = u64;
        #[cfg(fast_arithmetic = "32")]
        type Chunk = u32;
        const STEP: usize = mem::size_of::<Chunk>();
        const ONE_BYTES: Chunk = Chunk::MAX / 255;
        for chunk in rest.chunks_exact(STEP) {
            let chars = Chunk::from_le_bytes(chunk.try_into().unwrap());
            let contains_ctrl = chars.wrapping_sub(ONE_BYTES * 0x20) & !chars;
            let chars_quote = chars ^ (ONE_BYTES * Chunk::from(b'"'));
            let contains_quote = chars_quote.wrapping_sub(ONE_BYTES) & !chars_quote;
            let chars_backslash = chars ^ (ONE_BYTES * Chunk::from(b'\\'));
            let contains_backslash = chars_backslash.wrapping_sub(ONE_BYTES)
                & !chars_backslash;
            let masked = (contains_ctrl | contains_quote | contains_backslash)
                & (ONE_BYTES << 7);
            if masked != 0 {
                self.index = unsafe { chunk.as_ptr().offset_from(self.slice.as_ptr()) }
                    as usize + masked.trailing_zeros() as usize / 8;
                return;
            }
        }
        self.index += rest.len() / STEP * STEP;
        self.skip_to_escape_slow();
    }
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

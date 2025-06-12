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
    fn byte_offset(&self) -> usize {}
    fn parse_str<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, str>> {}
    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>> {}
    fn ignore_str(&mut self) -> Result<()> {
        loop {
            let ch = tri!(next_or_eof(self));
            if !is_escape(ch, true) {
                continue;
            }
            match ch {
                b'"' => {
                    return Ok(());
                }
                b'\\' => {
                    tri!(ignore_escape(self));
                }
                _ => {
                    return error(self, ErrorCode::ControlCharacterWhileParsingString);
                }
            }
        }
    }
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
fn is_escape(ch: u8, including_control_characters: bool) -> bool {
    ch == b'"' || ch == b'\\' || (including_control_characters && ch < 0x20)
}

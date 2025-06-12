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
#[cold]
fn parse_unicode_escape<'de, R: Read<'de>>(
    read: &mut R,
    validate: bool,
    scratch: &mut Vec<u8>,
) -> Result<()> {
    let mut n = tri!(read.decode_hex_escape());
    if validate && n >= 0xDC00 && n <= 0xDFFF {
        return error(read, ErrorCode::LoneLeadingSurrogateInHexEscape);
    }
    loop {
        if n < 0xD800 || n > 0xDBFF {
            push_wtf8_codepoint(n as u32, scratch);
            return Ok(());
        }
        let n1 = n;
        if tri!(peek_or_eof(read)) == b'\\' {
            read.discard();
        } else {
            return if validate {
                read.discard();
                error(read, ErrorCode::UnexpectedEndOfHexEscape)
            } else {
                push_wtf8_codepoint(n1 as u32, scratch);
                Ok(())
            };
        }
        if tri!(peek_or_eof(read)) == b'u' {
            read.discard();
        } else {
            return if validate {
                read.discard();
                error(read, ErrorCode::UnexpectedEndOfHexEscape)
            } else {
                push_wtf8_codepoint(n1 as u32, scratch);
                parse_escape(read, validate, scratch)
            };
        }
        let n2 = tri!(read.decode_hex_escape());
        if n2 < 0xDC00 || n2 > 0xDFFF {
            if validate {
                return error(read, ErrorCode::LoneLeadingSurrogateInHexEscape);
            }
            push_wtf8_codepoint(n1 as u32, scratch);
            n = n2;
            continue;
        }
        let n = ((((n1 - 0xD800) as u32) << 10) | (n2 - 0xDC00) as u32) + 0x1_0000;
        push_wtf8_codepoint(n, scratch);
        return Ok(());
    }
}
#[inline]
fn push_wtf8_codepoint(n: u32, scratch: &mut Vec<u8>) {
    if n < 0x80 {
        scratch.push(n as u8);
        return;
    }
    scratch.reserve(4);
    unsafe {
        let ptr = scratch.as_mut_ptr().add(scratch.len());
        let encoded_len = match n {
            0..=0x7F => unreachable!(),
            0x80..=0x7FF => {
                ptr.write(((n >> 6) & 0b0001_1111) as u8 | 0b1100_0000);
                2
            }
            0x800..=0xFFFF => {
                ptr.write(((n >> 12) & 0b0000_1111) as u8 | 0b1110_0000);
                ptr.add(1).write(((n >> 6) & 0b0011_1111) as u8 | 0b1000_0000);
                3
            }
            0x1_0000..=0x10_FFFF => {
                ptr.write(((n >> 18) & 0b0000_0111) as u8 | 0b1111_0000);
                ptr.add(1).write(((n >> 12) & 0b0011_1111) as u8 | 0b1000_0000);
                ptr.add(2).write(((n >> 6) & 0b0011_1111) as u8 | 0b1000_0000);
                4
            }
            0x11_0000.. => unreachable!(),
        };
        ptr.add(encoded_len - 1).write((n & 0b0011_1111) as u8 | 0b1000_0000);
        scratch.set_len(scratch.len() + encoded_len);
    }
}
fn peek_or_eof<'de, R>(read: &mut R) -> Result<u8>
where
    R: ?Sized + Read<'de>,
{
    match tri!(read.peek()) {
        Some(b) => Ok(b),
        None => error(read, ErrorCode::EofWhileParsingString),
    }
}
fn parse_escape<'de, R: Read<'de>>(
    read: &mut R,
    validate: bool,
    scratch: &mut Vec<u8>,
) -> Result<()> {
    let ch = tri!(next_or_eof(read));
    match ch {
        b'"' => scratch.push(b'"'),
        b'\\' => scratch.push(b'\\'),
        b'/' => scratch.push(b'/'),
        b'b' => scratch.push(b'\x08'),
        b'f' => scratch.push(b'\x0c'),
        b'n' => scratch.push(b'\n'),
        b'r' => scratch.push(b'\r'),
        b't' => scratch.push(b'\t'),
        b'u' => return parse_unicode_escape(read, validate, scratch),
        _ => return error(read, ErrorCode::InvalidEscape),
    }
    Ok(())
}

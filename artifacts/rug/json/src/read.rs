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

/// Trait used by the deserializer for iterating over input. This is manually
/// "specialized" for iterating over `&[u8]`. Once feature(specialization) is
/// stable we can use actual specialization.
///
/// This trait is sealed and cannot be implemented for types outside of
/// `serde_json`.
pub trait Read<'de>: private::Sealed {
    #[doc(hidden)]
    fn next(&mut self) -> Result<Option<u8>>;
    #[doc(hidden)]
    fn peek(&mut self) -> Result<Option<u8>>;

    /// Only valid after a call to peek(). Discards the peeked byte.
    #[doc(hidden)]
    fn discard(&mut self);

    /// Position of the most recent call to next().
    ///
    /// The most recent call was probably next() and not peek(), but this method
    /// should try to return a sensible result if the most recent call was
    /// actually peek() because we don't always know.
    ///
    /// Only called in case of an error, so performance is not important.
    #[doc(hidden)]
    fn position(&self) -> Position;

    /// Position of the most recent call to peek().
    ///
    /// The most recent call was probably peek() and not next(), but this method
    /// should try to return a sensible result if the most recent call was
    /// actually next() because we don't always know.
    ///
    /// Only called in case of an error, so performance is not important.
    #[doc(hidden)]
    fn peek_position(&self) -> Position;

    /// Offset from the beginning of the input to the next byte that would be
    /// returned by next() or peek().
    #[doc(hidden)]
    fn byte_offset(&self) -> usize;

    /// Assumes the previous byte was a quotation mark. Parses a JSON-escaped
    /// string until the next quotation mark using the given scratch space if
    /// necessary. The scratch space is initially empty.
    #[doc(hidden)]
    fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>>;

    /// Assumes the previous byte was a quotation mark. Parses a JSON-escaped
    /// string until the next quotation mark using the given scratch space if
    /// necessary. The scratch space is initially empty.
    ///
    /// This function returns the raw bytes in the string with escape sequences
    /// expanded but without performing unicode validation.
    #[doc(hidden)]
    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>>;

    /// Assumes the previous byte was a quotation mark. Parses a JSON-escaped
    /// string until the next quotation mark but discards the data.
    #[doc(hidden)]
    fn ignore_str(&mut self) -> Result<()>;

    /// Assumes the previous byte was a hex escape sequence ('\u') in a string.
    /// Parses next hexadecimal sequence.
    #[doc(hidden)]
    fn decode_hex_escape(&mut self) -> Result<u16>;

    /// Switch raw buffering mode on.
    ///
    /// This is used when deserializing `RawValue`.
    #[cfg(feature = "raw_value")]
    #[doc(hidden)]
    fn begin_raw_buffering(&mut self);

    /// Switch raw buffering mode off and provides the raw buffered data to the
    /// given visitor.
    #[cfg(feature = "raw_value")]
    #[doc(hidden)]
    fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>;

    /// Whether StreamDeserializer::next needs to check the failed flag. True
    /// for IoRead, false for StrRead and SliceRead which can track failure by
    /// truncating their input slice to avoid the extra check on every next
    /// call.
    #[doc(hidden)]
    const should_early_return_if_failed: bool;

    /// Mark a persistent failure of StreamDeserializer, either by setting the
    /// flag or by truncating the input data.
    #[doc(hidden)]
    fn set_failed(&mut self, failed: &mut bool);
}

pub struct Position {
    pub line: usize,
    pub column: usize,
}

pub enum Reference<'b, 'c, T>
where
    T: ?Sized + 'static,
{
    Borrowed(&'b T),
    Copied(&'c T),
}

impl<'b, 'c, T> Deref for Reference<'b, 'c, T>
where
    T: ?Sized + 'static,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match *self {
            Reference::Borrowed(b) => b,
            Reference::Copied(c) => c,
        }
    }
}

/// JSON input source that reads from a std::io input stream.
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

/// JSON input source that reads from a slice of bytes.
//
// This is more efficient than other iterators because peek() can be read-only
// and we can compute line/col position only if an error happens.
pub struct SliceRead<'a> {
    slice: &'a [u8],
    /// Index of the *next* byte that will be returned by next() or peek().
    index: usize,
    #[cfg(feature = "raw_value")]
    raw_buffering_start_index: usize,
}

/// JSON input source that reads from a UTF-8 string.
//
// Able to elide UTF-8 checks by assuming that the input is valid UTF-8.
pub struct StrRead<'a> {
    delegate: SliceRead<'a>,
    #[cfg(feature = "raw_value")]
    data: &'a str,
}

// Prevent users from implementing the Read trait.
mod private {
    pub trait Sealed {}
}

//////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "std")]
impl<R> IoRead<R>
where
    R: io::Read,
{
    /// Create a JSON input source to read from a std::io input stream.
    ///
    /// When reading from a source against which short reads are not efficient, such
    /// as a [`File`], you will want to apply your own buffering because serde_json
    /// will not buffer the input. See [`std::io::BufReader`].
    ///
    /// [`File`]: std::fs::File
    pub fn new(reader: R) -> Self {
        IoRead {
            iter: LineColIterator::new(reader.bytes()),
            ch: None,
            #[cfg(feature = "raw_value")]
            raw_buffer: None,
        }
    }
}

#[cfg(feature = "std")]
impl<R> private::Sealed for IoRead<R> where R: io::Read {}

#[cfg(feature = "std")]
impl<R> IoRead<R>
where
    R: io::Read,
{
    fn parse_str_bytes<'s, T, F>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
        validate: bool,
        result: F,
    ) -> Result<T>
    where
        T: 's,
        F: FnOnce(&'s Self, &'s [u8]) -> Result<T>,
    {
        loop {
            let ch = tri!(next_or_eof(self));
            if !is_escape(ch, true) {
                scratch.push(ch);
                continue;
            }
            match ch {
                b'"' => {
                    return result(self, scratch);
                }
                b'\\' => {
                    tri!(parse_escape(self, validate, scratch));
                }
                _ => {
                    if validate {
                        return error(self, ErrorCode::ControlCharacterWhileParsingString);
                    }
                    scratch.push(ch);
                }
            }
        }
    }
}

#[cfg(feature = "std")]
impl<'de, R> Read<'de> for IoRead<R>
where
    R: io::Read,
{
    #[inline]
    fn next(&mut self) -> Result<Option<u8>> {
        match self.ch.take() {
            Some(ch) => {
                #[cfg(feature = "raw_value")]
                {
                    if let Some(buf) = &mut self.raw_buffer {
                        buf.push(ch);
                    }
                }
                Ok(Some(ch))
            }
            None => match self.iter.next() {
                Some(Err(err)) => Err(Error::io(err)),
                Some(Ok(ch)) => {
                    #[cfg(feature = "raw_value")]
                    {
                        if let Some(buf) = &mut self.raw_buffer {
                            buf.push(ch);
                        }
                    }
                    Ok(Some(ch))
                }
                None => Ok(None),
            },
        }
    }

    #[inline]
    fn peek(&mut self) -> Result<Option<u8>> {
        match self.ch {
            Some(ch) => Ok(Some(ch)),
            None => match self.iter.next() {
                Some(Err(err)) => Err(Error::io(err)),
                Some(Ok(ch)) => {
                    self.ch = Some(ch);
                    Ok(self.ch)
                }
                None => Ok(None),
            },
        }
    }

    #[cfg(not(feature = "raw_value"))]
    #[inline]
    fn discard(&mut self) {
        self.ch = None;
    }

    #[cfg(feature = "raw_value")]
    fn discard(&mut self) {
        if let Some(ch) = self.ch.take() {
            if let Some(buf) = &mut self.raw_buffer {
                buf.push(ch);
            }
        }
    }

    fn position(&self) -> Position {
        Position {
            line: self.iter.line(),
            column: self.iter.col(),
        }
    }

    fn peek_position(&self) -> Position {
        // The LineColIterator updates its position during peek() so it has the
        // right one here.
        self.position()
    }

    fn byte_offset(&self) -> usize {
        match self.ch {
            Some(_) => self.iter.byte_offset() - 1,
            None => self.iter.byte_offset(),
        }
    }

    fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
        self.parse_str_bytes(scratch, true, as_str)
            .map(Reference::Copied)
    }

    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>> {
        self.parse_str_bytes(scratch, false, |_, bytes| Ok(bytes))
            .map(Reference::Copied)
    }

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

    fn decode_hex_escape(&mut self) -> Result<u16> {
        let a = tri!(next_or_eof(self));
        let b = tri!(next_or_eof(self));
        let c = tri!(next_or_eof(self));
        let d = tri!(next_or_eof(self));
        match decode_four_hex_digits(a, b, c, d) {
            Some(val) => Ok(val),
            None => error(self, ErrorCode::InvalidEscape),
        }
    }

    #[cfg(feature = "raw_value")]
    fn begin_raw_buffering(&mut self) {
        self.raw_buffer = Some(Vec::new());
    }

    #[cfg(feature = "raw_value")]
    fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let raw = self.raw_buffer.take().unwrap();
        let raw = match String::from_utf8(raw) {
            Ok(raw) => raw,
            Err(_) => return error(self, ErrorCode::InvalidUnicodeCodePoint),
        };
        visitor.visit_map(OwnedRawDeserializer {
            raw_value: Some(raw),
        })
    }

    const should_early_return_if_failed: bool = true;

    #[inline]
    #[cold]
    fn set_failed(&mut self, failed: &mut bool) {
        *failed = true;
    }
}

//////////////////////////////////////////////////////////////////////////////

impl<'a> SliceRead<'a> {
    /// Create a JSON input source to read from a slice of bytes.
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

    fn skip_to_escape(&mut self, forbid_control_characters: bool) {
        // Immediately bail-out on empty strings and consecutive escapes (e.g. \u041b\u0435)
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

        // We wish to find the first byte in range 0x00..=0x1F or " or \. Ideally, we'd use
        // something akin to memchr3, but the memchr crate does not support this at the moment.
        // Therefore, we use a variation on Mycroft's algorithm [1] to provide performance better
        // than a naive loop. It runs faster than equivalent two-pass memchr2+SWAR code on
        // benchmarks and it's cross-platform, so probably the right fit.
        // [1]: https://groups.google.com/forum/#!original/comp.lang.c/2HtQXvg7iKc/xOJeipH6KLMJ

        #[cfg(fast_arithmetic = "64")]
        type Chunk = u64;
        #[cfg(fast_arithmetic = "32")]
        type Chunk = u32;

        const STEP: usize = mem::size_of::<Chunk>();
        const ONE_BYTES: Chunk = Chunk::MAX / 255; // 0x0101...01

        for chunk in rest.chunks_exact(STEP) {
            let chars = Chunk::from_le_bytes(chunk.try_into().unwrap());
            let contains_ctrl = chars.wrapping_sub(ONE_BYTES * 0x20) & !chars;
            let chars_quote = chars ^ (ONE_BYTES * Chunk::from(b'"'));
            let contains_quote = chars_quote.wrapping_sub(ONE_BYTES) & !chars_quote;
            let chars_backslash = chars ^ (ONE_BYTES * Chunk::from(b'\\'));
            let contains_backslash = chars_backslash.wrapping_sub(ONE_BYTES) & !chars_backslash;
            let masked = (contains_ctrl | contains_quote | contains_backslash) & (ONE_BYTES << 7);
            if masked != 0 {
                // SAFETY: chunk is in-bounds for slice
                self.index = unsafe { chunk.as_ptr().offset_from(self.slice.as_ptr()) } as usize
                    + masked.trailing_zeros() as usize / 8;
                return;
            }
        }

        self.index += rest.len() / STEP * STEP;
        self.skip_to_escape_slow();
    }

    #[cold]
    #[inline(never)]
    fn skip_to_escape_slow(&mut self) {
        while self.index < self.slice.len() && !is_escape(self.slice[self.index], true) {
            self.index += 1;
        }
    }

    /// The big optimization here over IoRead is that if the string contains no
    /// backslash escape sequences, the returned &str is a slice of the raw JSON
    /// data so we avoid copying into the scratch space.
    fn parse_str_bytes<'s, T, F>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
        validate: bool,
        result: F,
    ) -> Result<Reference<'a, 's, T>>
    where
        T: ?Sized + 's,
        F: for<'f> FnOnce(&'s Self, &'f [u8]) -> Result<&'f T>,
    {
        // Index of the first byte not yet copied into the scratch space.
        let mut start = self.index;

        loop {
            self.skip_to_escape(validate);
            if self.index == self.slice.len() {
                return error(self, ErrorCode::EofWhileParsingString);
            }
            match self.slice[self.index] {
                b'"' => {
                    if scratch.is_empty() {
                        // Fast path: return a slice of the raw JSON without any
                        // copying.
                        let borrowed = &self.slice[start..self.index];
                        self.index += 1;
                        return result(self, borrowed).map(Reference::Borrowed);
                    } else {
                        scratch.extend_from_slice(&self.slice[start..self.index]);
                        self.index += 1;
                        return result(self, scratch).map(Reference::Copied);
                    }
                }
                b'\\' => {
                    scratch.extend_from_slice(&self.slice[start..self.index]);
                    self.index += 1;
                    tri!(parse_escape(self, validate, scratch));
                    start = self.index;
                }
                _ => {
                    self.index += 1;
                    return error(self, ErrorCode::ControlCharacterWhileParsingString);
                }
            }
        }
    }
}

impl<'a> private::Sealed for SliceRead<'a> {}

impl<'a> Read<'a> for SliceRead<'a> {
    #[inline]
    fn next(&mut self) -> Result<Option<u8>> {
        // `Ok(self.slice.get(self.index).map(|ch| { self.index += 1; *ch }))`
        // is about 10% slower.
        Ok(if self.index < self.slice.len() {
            let ch = self.slice[self.index];
            self.index += 1;
            Some(ch)
        } else {
            None
        })
    }

    #[inline]
    fn peek(&mut self) -> Result<Option<u8>> {
        // `Ok(self.slice.get(self.index).map(|ch| *ch))` is about 10% slower
        // for some reason.
        Ok(if self.index < self.slice.len() {
            Some(self.slice[self.index])
        } else {
            None
        })
    }

    #[inline]
    fn discard(&mut self) {
        self.index += 1;
    }

    fn position(&self) -> Position {
        self.position_of_index(self.index)
    }

    fn peek_position(&self) -> Position {
        // Cap it at slice.len() just in case the most recent call was next()
        // and it returned the last byte.
        self.position_of_index(cmp::min(self.slice.len(), self.index + 1))
    }

    fn byte_offset(&self) -> usize {
        self.index
    }

    fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, str>> {
        self.parse_str_bytes(scratch, true, as_str)
    }

    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'a, 's, [u8]>> {
        self.parse_str_bytes(scratch, false, |_, bytes| Ok(bytes))
    }

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
    fn decode_hex_escape(&mut self) -> Result<u16> {
        match self.slice[self.index..] {
            [a, b, c, d, ..] => {
                self.index += 4;
                match decode_four_hex_digits(a, b, c, d) {
                    Some(val) => Ok(val),
                    None => error(self, ErrorCode::InvalidEscape),
                }
            }
            _ => {
                self.index = self.slice.len();
                error(self, ErrorCode::EofWhileParsingString)
            }
        }
    }

    #[cfg(feature = "raw_value")]
    fn begin_raw_buffering(&mut self) {
        self.raw_buffering_start_index = self.index;
    }

    #[cfg(feature = "raw_value")]
    fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'a>,
    {
        let raw = &self.slice[self.raw_buffering_start_index..self.index];
        let raw = match str::from_utf8(raw) {
            Ok(raw) => raw,
            Err(_) => return error(self, ErrorCode::InvalidUnicodeCodePoint),
        };
        visitor.visit_map(BorrowedRawDeserializer {
            raw_value: Some(raw),
        })
    }

    const should_early_return_if_failed: bool = false;

    #[inline]
    #[cold]
    fn set_failed(&mut self, _failed: &mut bool) {
        self.slice = &self.slice[..self.index];
    }
}

//////////////////////////////////////////////////////////////////////////////

impl<'a> StrRead<'a> {
    /// Create a JSON input source to read from a UTF-8 string.
    pub fn new(s: &'a str) -> Self {
        StrRead {
            delegate: SliceRead::new(s.as_bytes()),
            #[cfg(feature = "raw_value")]
            data: s,
        }
    }
}

impl<'a> private::Sealed for StrRead<'a> {}

impl<'a> Read<'a> for StrRead<'a> {
    #[inline]
    fn next(&mut self) -> Result<Option<u8>> {
        self.delegate.next()
    }

    #[inline]
    fn peek(&mut self) -> Result<Option<u8>> {
        self.delegate.peek()
    }

    #[inline]
    fn discard(&mut self) {
        self.delegate.discard();
    }

    fn position(&self) -> Position {
        self.delegate.position()
    }

    fn peek_position(&self) -> Position {
        self.delegate.peek_position()
    }

    fn byte_offset(&self) -> usize {
        self.delegate.byte_offset()
    }

    fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, str>> {
        self.delegate.parse_str_bytes(scratch, true, |_, bytes| {
            // The deserialization input came in as &str with a UTF-8 guarantee,
            // and the \u-escapes are checked along the way, so don't need to
            // check here.
            Ok(unsafe { str::from_utf8_unchecked(bytes) })
        })
    }

    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'a, 's, [u8]>> {
        self.delegate.parse_str_raw(scratch)
    }

    fn ignore_str(&mut self) -> Result<()> {
        self.delegate.ignore_str()
    }

    fn decode_hex_escape(&mut self) -> Result<u16> {
        self.delegate.decode_hex_escape()
    }

    #[cfg(feature = "raw_value")]
    fn begin_raw_buffering(&mut self) {
        self.delegate.begin_raw_buffering();
    }

    #[cfg(feature = "raw_value")]
    fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'a>,
    {
        let raw = &self.data[self.delegate.raw_buffering_start_index..self.delegate.index];
        visitor.visit_map(BorrowedRawDeserializer {
            raw_value: Some(raw),
        })
    }

    const should_early_return_if_failed: bool = false;

    #[inline]
    #[cold]
    fn set_failed(&mut self, failed: &mut bool) {
        self.delegate.set_failed(failed);
    }
}

//////////////////////////////////////////////////////////////////////////////

impl<'de, R> private::Sealed for &mut R where R: Read<'de> {}

impl<'de, R> Read<'de> for &mut R
where
    R: Read<'de>,
{
    fn next(&mut self) -> Result<Option<u8>> {
        R::next(self)
    }

    fn peek(&mut self) -> Result<Option<u8>> {
        R::peek(self)
    }

    fn discard(&mut self) {
        R::discard(self);
    }

    fn position(&self) -> Position {
        R::position(self)
    }

    fn peek_position(&self) -> Position {
        R::peek_position(self)
    }

    fn byte_offset(&self) -> usize {
        R::byte_offset(self)
    }

    fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
        R::parse_str(self, scratch)
    }

    fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>> {
        R::parse_str_raw(self, scratch)
    }

    fn ignore_str(&mut self) -> Result<()> {
        R::ignore_str(self)
    }

    fn decode_hex_escape(&mut self) -> Result<u16> {
        R::decode_hex_escape(self)
    }

    #[cfg(feature = "raw_value")]
    fn begin_raw_buffering(&mut self) {
        R::begin_raw_buffering(self);
    }

    #[cfg(feature = "raw_value")]
    fn end_raw_buffering<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        R::end_raw_buffering(self, visitor)
    }

    const should_early_return_if_failed: bool = R::should_early_return_if_failed;

    fn set_failed(&mut self, failed: &mut bool) {
        R::set_failed(self, failed);
    }
}

//////////////////////////////////////////////////////////////////////////////

/// Marker for whether StreamDeserializer can implement FusedIterator.
pub trait Fused: private::Sealed {}
impl<'a> Fused for SliceRead<'a> {}
impl<'a> Fused for StrRead<'a> {}

fn is_escape(ch: u8, including_control_characters: bool) -> bool {
    ch == b'"' || ch == b'\\' || (including_control_characters && ch < 0x20)
}

fn next_or_eof<'de, R>(read: &mut R) -> Result<u8>
where
    R: ?Sized + Read<'de>,
{
    match tri!(read.next()) {
        Some(b) => Ok(b),
        None => error(read, ErrorCode::EofWhileParsingString),
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

fn error<'de, R, T>(read: &R, reason: ErrorCode) -> Result<T>
where
    R: ?Sized + Read<'de>,
{
    let position = read.position();
    Err(Error::syntax(reason, position.line, position.column))
}

fn as_str<'de, 's, R: Read<'de>>(read: &R, slice: &'s [u8]) -> Result<&'s str> {
    str::from_utf8(slice).or_else(|_| error(read, ErrorCode::InvalidUnicodeCodePoint))
}

/// Parses a JSON escape sequence and appends it into the scratch space. Assumes
/// the previous byte read was a backslash.
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

/// Parses a JSON \u escape and appends it into the scratch space. Assumes `\u`
/// has just been read.
#[cold]
fn parse_unicode_escape<'de, R: Read<'de>>(
    read: &mut R,
    validate: bool,
    scratch: &mut Vec<u8>,
) -> Result<()> {
    let mut n = tri!(read.decode_hex_escape());

    // Non-BMP characters are encoded as a sequence of two hex escapes,
    // representing UTF-16 surrogates. If deserializing a utf-8 string the
    // surrogates are required to be paired, whereas deserializing a byte string
    // accepts lone surrogates.
    if validate && n >= 0xDC00 && n <= 0xDFFF {
        // XXX: This is actually a trailing surrogate.
        return error(read, ErrorCode::LoneLeadingSurrogateInHexEscape);
    }

    loop {
        if n < 0xD800 || n > 0xDBFF {
            // Every u16 outside of the surrogate ranges is guaranteed to be a
            // legal char.
            push_wtf8_codepoint(n as u32, scratch);
            return Ok(());
        }

        // n is a leading surrogate, we now expect a trailing surrogate.
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
                // The \ prior to this byte started an escape sequence, so we
                // need to parse that now. This recursive call does not blow the
                // stack on malicious input because the escape is not \u, so it
                // will be handled by one of the easy nonrecursive cases.
                parse_escape(read, validate, scratch)
            };
        }

        let n2 = tri!(read.decode_hex_escape());

        if n2 < 0xDC00 || n2 > 0xDFFF {
            if validate {
                return error(read, ErrorCode::LoneLeadingSurrogateInHexEscape);
            }
            push_wtf8_codepoint(n1 as u32, scratch);
            // If n2 is a leading surrogate, we need to restart.
            n = n2;
            continue;
        }

        // This value is in range U+10000..=U+10FFFF, which is always a valid
        // codepoint.
        let n = ((((n1 - 0xD800) as u32) << 10) | (n2 - 0xDC00) as u32) + 0x1_0000;
        push_wtf8_codepoint(n, scratch);
        return Ok(());
    }
}

/// Adds a WTF-8 codepoint to the end of the buffer. This is a more efficient
/// implementation of String::push. The codepoint may be a surrogate.
#[inline]
fn push_wtf8_codepoint(n: u32, scratch: &mut Vec<u8>) {
    if n < 0x80 {
        scratch.push(n as u8);
        return;
    }

    scratch.reserve(4);

    // SAFETY: After the `reserve` call, `scratch` has at least 4 bytes of
    // allocated but uninitialized memory after its last initialized byte, which
    // is where `ptr` points. All reachable match arms write `encoded_len` bytes
    // to that region and update the length accordingly, and `encoded_len` is
    // always <= 4.
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
                ptr.add(1)
                    .write(((n >> 6) & 0b0011_1111) as u8 | 0b1000_0000);
                3
            }
            0x1_0000..=0x10_FFFF => {
                ptr.write(((n >> 18) & 0b0000_0111) as u8 | 0b1111_0000);
                ptr.add(1)
                    .write(((n >> 12) & 0b0011_1111) as u8 | 0b1000_0000);
                ptr.add(2)
                    .write(((n >> 6) & 0b0011_1111) as u8 | 0b1000_0000);
                4
            }
            0x11_0000.. => unreachable!(),
        };
        ptr.add(encoded_len - 1)
            .write((n & 0b0011_1111) as u8 | 0b1000_0000);

        scratch.set_len(scratch.len() + encoded_len);
    }
}

/// Parses a JSON escape sequence and discards the value. Assumes the previous
/// byte read was a backslash.
fn ignore_escape<'de, R>(read: &mut R) -> Result<()>
where
    R: ?Sized + Read<'de>,
{
    let ch = tri!(next_or_eof(read));

    match ch {
        b'"' | b'\\' | b'/' | b'b' | b'f' | b'n' | b'r' | b't' => {}
        b'u' => {
            // At this point we don't care if the codepoint is valid. We just
            // want to consume it. We don't actually know what is valid or not
            // at this point, because that depends on if this string will
            // ultimately be parsed into a string or a byte buffer in the "real"
            // parse.

            tri!(read.decode_hex_escape());
        }
        _ => {
            return error(read, ErrorCode::InvalidEscape);
        }
    }

    Ok(())
}

const fn decode_hex_val_slow(val: u8) -> Option<u8> {
    match val {
        b'0'..=b'9' => Some(val - b'0'),
        b'A'..=b'F' => Some(val - b'A' + 10),
        b'a'..=b'f' => Some(val - b'a' + 10),
        _ => None,
    }
}

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

static HEX0: [i16; 256] = build_hex_table(0);
static HEX1: [i16; 256] = build_hex_table(4);

fn decode_four_hex_digits(a: u8, b: u8, c: u8, d: u8) -> Option<u16> {
    let a = HEX1[a as usize] as i32;
    let b = HEX0[b as usize] as i32;
    let c = HEX1[c as usize] as i32;
    let d = HEX0[d as usize] as i32;

    let codepoint = ((a | b) << 8) | c | d;

    // A single sign bit check.
    if codepoint >= 0 {
        Some(codepoint as u16)
    } else {
        None
    }
}

#[cfg(test)]
mod tests_llm_16_45 {
    use super::*;

use crate::*;
    use crate::de::{Read, Deserializer};
    use std::io::Cursor;

    #[test]
    fn test_peek_some_byte() {
        let data = b"test";
        let mut cursor = Cursor::new(data);
        let mut deserializer = Deserializer::from_reader(&mut cursor);
        
        let result = deserializer.peek();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some(b't'));
    }

    #[test]
    fn test_peek_none_byte() {
        let data = b"";
        let mut cursor = Cursor::new(data);
        let mut deserializer = Deserializer::from_reader(&mut cursor);
        
        let result = deserializer.peek();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), None);
    }
}

#[cfg(test)]
mod tests_llm_16_275 {
    use super::*;

use crate::*;
    use std::io::Cursor;

    #[test]
    fn test_decode_hex_escape_valid() {
        let input = b"\\u003a"; // Hex representation for ':'
        let mut reader = IoRead::new(Cursor::new(input));
        let result = reader.decode_hex_escape();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 0x003a);
    }

    #[test]
    fn test_decode_hex_escape_invalid() {
        let input = b"\\u003g"; // Invalid hex character
        let mut reader = IoRead::new(Cursor::new(input));
        let result = reader.decode_hex_escape();
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_hex_escape_incomplete() {
        let input = b"\\u00"; // Incomplete hex digits
        let mut reader = IoRead::new(Cursor::new(input));
        let result = reader.decode_hex_escape();
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod tests_llm_16_276 {
    use super::*;

use crate::*;
    use std::io::Cursor;
    use crate::read::IoRead;

    #[test]
    fn test_discard() {
        let input = Cursor::new(b"Hello, World!\n");
        let mut io_read = IoRead::new(input);

        // Read a character to set `ch`
        let _ = io_read.next().unwrap();

        // Ensure `ch` is set before discard
        assert!(io_read.ch.is_some());

        // Discard the character
        io_read.discard();

        // Ensure `ch` is None after discard
        assert!(io_read.ch.is_none());
    }
}

#[cfg(test)]
mod tests_llm_16_277 {
    use super::*;

use crate::*;
    use std::io::Cursor;

    #[test]
    fn test_ignore_str_valid_string() {
        let input = b"\"Hello, World!\"";
        let mut reader = IoRead::new(Cursor::new(input));
        
        let result = reader.ignore_str();
        assert!(result.is_ok());
    }

    #[test]
    fn test_ignore_str_with_escape() {
        let input = b"\"Hello, \\\"World!\\\"\"";
        let mut reader = IoRead::new(Cursor::new(input));
        
        let result = reader.ignore_str();
        assert!(result.is_ok());
    }

    #[test]
    fn test_ignore_str_invalid_control_character() {
        let input = b"\"Hello, \x01 World!\"";
        let mut reader = IoRead::new(Cursor::new(input));
        
        let result = reader.ignore_str();
        assert!(result.is_err());
    }

    #[test]
    fn test_ignore_str_multiple_escapes() {
        let input = b"\"Hello, \\\\ World!\"";
        let mut reader = IoRead::new(Cursor::new(input));
        
        let result = reader.ignore_str();
        assert!(result.is_ok());
    }

    #[test]
    fn test_ignore_str_empty_string() {
        let input = b"\"\"";
        let mut reader = IoRead::new(Cursor::new(input));
        
        let result = reader.ignore_str();
        assert!(result.is_ok());
    }

    #[test]
    fn test_ignore_str_end_of_input() {
        let input = b"\"Hello, World!"; // Missing closing quote
        let mut reader = IoRead::new(Cursor::new(input));
        
        let result = reader.ignore_str();
        assert!(result.is_err());
    }
}

#[cfg(test)]
mod tests_llm_16_282 {
    use super::*;

use crate::*;
    use std::io::Cursor;

    #[test]
    fn test_peek_position() {
        let data = b"Hello\nWorld\n";
        let cursor = Cursor::new(data);
        let mut io_read = IoRead::new(cursor);

        // Read a single byte to update the position.
        let _ = io_read.next().unwrap();
        let position = io_read.peek_position();
        assert_eq!(position.line, 1);
        assert_eq!(position.column, 1);

        // Peek another byte
        let _ = io_read.peek().unwrap();
        let position = io_read.peek_position();
        assert_eq!(position.line, 1);
        assert_eq!(position.column, 2);

        // Read line ending
        let _ = io_read.next().unwrap(); // Read 'e'
        let _ = io_read.next().unwrap(); // Read 'l'
        let _ = io_read.next().unwrap(); // Read 'l'
        let _ = io_read.next().unwrap(); // Read 'o'
        let _ = io_read.next().unwrap(); // Read '\n'
        let position = io_read.peek_position();
        assert_eq!(position.line, 2);
        assert_eq!(position.column, 1);
    }
}

#[cfg(test)]
mod tests_llm_16_283 {
    use super::*;

use crate::*;
    use std::io::Cursor;

    #[test]
    fn test_position_initial() {
        let data = b"Hello, World\nThis is a test.";
        let reader = Cursor::new(data);
        let mut json_reader = IoRead::new(reader);
        
        let pos = json_reader.position();
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 0);
    }

    #[test]
    fn test_position_after_reading() {
        let data = b"Hello, World\nThis is a test.";
        let reader = Cursor::new(data);
        let mut json_reader = IoRead::new(reader);

        // Read a byte
        let _ = json_reader.next().unwrap();
        let pos = json_reader.position();
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 1);

        // Read until newline
        for _ in 0..12 {
            let _ = json_reader.next().unwrap();
        }
        let pos = json_reader.position();
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 13);

        // Read the newline
        let _ = json_reader.next().unwrap();
        let pos = json_reader.position();
        assert_eq!(pos.line, 2);
        assert_eq!(pos.column, 0);
    }

    #[test]
    fn test_position_end_of_input() {
        let data = b"Hello\nWorld";
        let reader = Cursor::new(data);
        let mut json_reader = IoRead::new(reader);

        // Read until the end
        let _ = json_reader.next();
        let _ = json_reader.next();
        let _ = json_reader.next();
        let _ = json_reader.next();
        let _ = json_reader.next();
        
        let pos = json_reader.position();
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 5);

        // Read newline
        let _ = json_reader.next();
        
        let _ = json_reader.next();  // Read 'W'
        let pos = json_reader.position();
        assert_eq!(pos.line, 2);
        assert_eq!(pos.column, 1);

        // Read until the end
        let _ = json_reader.next();
        let _ = json_reader.next();
        let _ = json_reader.next();
        let _ = json_reader.next();
        
        let pos = json_reader.position();
        assert_eq!(pos.line, 2);
        assert_eq!(pos.column, 5);
    }
}

#[cfg(test)]
mod tests_llm_16_285 {
    use super::*;

use crate::*;
    use crate::read::Reference;

    #[test]
    fn test_deref_borrowed() {
        let value = 42;
        let reference = Reference::Borrowed(&value);
        assert_eq!(*reference.deref(), value);
    }

    #[test]
    fn test_deref_copied() {
        let value = 42;
        let reference = Reference::Copied(&value);
        assert_eq!(*reference.deref(), value);
    }
}

#[cfg(test)]
mod tests_llm_16_286 {
    use super::*;

use crate::*;
    use crate::read::SliceRead;

    #[test]
    fn test_byte_offset() {
        let data = b"Hello, World!";
        let mut reader = SliceRead::new(data);

        // Initial byte offset should be 0
        assert_eq!(reader.byte_offset(), 0);

        // Read one byte
        let _ = reader.next().unwrap();
        assert_eq!(reader.byte_offset(), 1);

        // Read another byte
        let _ = reader.next().unwrap();
        assert_eq!(reader.byte_offset(), 2);

        // Move to the end
        while reader.next().unwrap().is_some() {}

        // Final byte offset should be equal to the length of the data
        assert_eq!(reader.byte_offset(), data.len());
    }
}

#[cfg(test)]
mod tests_llm_16_288 {
    use super::*;

use crate::*;
    use crate::read::SliceRead;

    #[test]
    fn test_discard() {
        let mut data = [1, 2, 3, 4, 5];
        let mut reader = SliceRead::new(&data);
        
        // Initial index should be 0
        assert_eq!(reader.index, 0);

        // Discard first byte
        reader.discard();
        assert_eq!(reader.index, 1);

        // Discard second byte
        reader.discard();
        assert_eq!(reader.index, 2);

        // Discard third byte
        reader.discard();
        assert_eq!(reader.index, 3);

        // Discard fourth byte
        reader.discard();
        assert_eq!(reader.index, 4);

        // Discard fifth byte
        reader.discard();
        assert_eq!(reader.index, 5);

        // After discarding all bytes, index should equal the length of the data
        assert_eq!(reader.index, data.len());
    }
}

#[cfg(test)]
mod tests_llm_16_294 {
    use super::*;

use crate::*;
    use crate::read::SliceRead;

    #[test]
    fn test_peek_position() {
        let slice = b"Hello\nWorld\n!";
        let mut reader = SliceRead::new(slice);

        // Test at the beginning
        let pos = reader.peek_position();
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 0);

        // Read one byte
        reader.next().unwrap();
        let pos = reader.peek_position();
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 1);

        // Read until the end of the first line
        reader.next().unwrap(); // Read 'e'
        reader.next().unwrap(); // Read 'l'
        reader.next().unwrap(); // Read 'l'
        reader.next().unwrap(); // Read 'o'
        reader.next().unwrap(); // Read '\n'
        let pos = reader.peek_position();
        assert_eq!(pos.line, 2);
        assert_eq!(pos.column, 0);

        // Read next line
        reader.next().unwrap(); // Read 'W'
        let pos = reader.peek_position();
        assert_eq!(pos.line, 2);
        assert_eq!(pos.column, 1);

        // Read until the end
        reader.next().unwrap(); // Read 'o'
        reader.next().unwrap(); // Read 'r'
        reader.next().unwrap(); // Read 'l'
        reader.next().unwrap(); // Read 'd'
        reader.next().unwrap(); // Read '\n'
        let pos = reader.peek_position();
        assert_eq!(pos.line, 3);
        assert_eq!(pos.column, 0);

        // Last line
        reader.next().unwrap(); // Read '!'
        let pos = reader.peek_position();
        assert_eq!(pos.line, 3);
        assert_eq!(pos.column, 1);

        // Try peeking when at the end
        reader.next(); // Move to end
        let pos = reader.peek_position();
        assert_eq!(pos.line, 3);
        assert_eq!(pos.column, 2);
    }
}

#[cfg(test)]
mod tests_llm_16_295 {
    use super::*;

use crate::*;
    use crate::read::SliceRead;

    #[test]
    fn test_position() {
        let slice: &[u8] = b"{\"key\": \"value\"}";
        let mut reader = SliceRead::new(slice);

        // Check position at the start
        let pos = reader.position();
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 0);

        // Move the reader ahead
        reader.next().unwrap(); // Read '{'
        
        // Check position after reading the first character
        let pos = reader.position();
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 1);

        // Read more characters
        reader.next().unwrap(); // Read '"'
        reader.next().unwrap(); // Read 'k'
        reader.next().unwrap(); // Read 'e'
        reader.next().unwrap(); // Read 'y'
        reader.next().unwrap(); // Read '"

        // Check position after reading "key"
        let pos = reader.position();
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 6);

        // Read the rest of the slice
        while reader.next().unwrap().is_some() {};

        // Check position at the end of the input
        let pos = reader.position();
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, slice.len()); // Should equal the total length
    }
}

#[cfg(test)]
mod tests_llm_16_296 {
    use super::*;

use crate::*;
    use crate::read::SliceRead;

    #[test]
    fn test_set_failed() {
        let mut slice_read = SliceRead::new(b"Test slice data.");
        slice_read.index = 5; // Set index to a position in the slice
        let mut failed = false;

        // Call the `set_failed` function
        slice_read.set_failed(&mut failed);

        // Check if the slice was updated correctly
        assert_eq!(slice_read.slice, b"Test s"); // Should only include data up to index 5
        assert_eq!(slice_read.index, 5); // Index should remain the same
    }
}

#[cfg(test)]
mod tests_llm_16_297 {
    use super::*;

use crate::*;
    use crate::read::{StrRead, SliceRead};

    #[test]
    fn test_byte_offset() {
        let json_str = "hello";
        let str_read = StrRead::new(json_str);

        // The initial byte offset should be 0
        assert_eq!(str_read.byte_offset(), 0);

        // Create a mutable instance to test the next and byte_offset
        let mut str_read = StrRead::new(json_str);

        // Read the first byte
        str_read.next().unwrap();
        assert_eq!(str_read.byte_offset(), 1);

        // Read the second byte
        str_read.next().unwrap();
        assert_eq!(str_read.byte_offset(), 2);

        // Read the third byte
        str_read.next().unwrap();
        assert_eq!(str_read.byte_offset(), 3);

        // Read the fourth byte
        str_read.next().unwrap();
        assert_eq!(str_read.byte_offset(), 4);

        // Read the fifth byte
        str_read.next().unwrap();
        assert_eq!(str_read.byte_offset(), 5);

        // After reading all bytes, next should return None
        assert_eq!(str_read.next().unwrap(), None);
        assert_eq!(str_read.byte_offset(), 5);
    }
}

#[cfg(test)]
mod tests_llm_16_298 {
    use super::*;

use crate::*;
    use crate::read::{StrRead, SliceRead};
    use crate::Result;

    #[test]
    fn test_decode_hex_escape() {
        let hex_escape = "\\u0041"; // Represents 'A' in unicode
        let mut reader = StrRead::new(hex_escape);
        
        // Decode the hex escape
        let result: Result<u16> = reader.decode_hex_escape();
        assert_eq!(result.unwrap(), 0x0041); // Check if it decodes to the correct unicode value
    }

    #[test]
    fn test_decode_hex_escape_invalid() {
        let hex_escape_invalid = "\\uZZZZ"; // Invalid hex escape
        let mut reader = StrRead::new(hex_escape_invalid);
        
        // Attempt to decode the hex escape
        let result: Result<u16> = reader.decode_hex_escape();
        assert!(result.is_err()); // Check if it returns an error
    }

    #[test]
    fn test_decode_hex_escape_eof() {
        let hex_escape_eof = "\\u"; // Incomplete hex escape
        let mut reader = StrRead::new(hex_escape_eof);
        
        // Attempt to decode the hex escape
        let result: Result<u16> = reader.decode_hex_escape();
        assert!(result.is_err()); // Check if it returns an error
    }
}

#[cfg(test)]
mod tests_llm_16_299 {
    use super::*;

use crate::*;
    use crate::read::{SliceRead, StrRead};

    #[test]
    fn test_strread_discard() {
        let json_string = r#"{ "key": "value" }"#;
        let mut str_read = StrRead::new(json_string);
        
        // Read first character
        assert_eq!(str_read.next().unwrap().unwrap(), b'{');

        // Discard the first character
        str_read.discard();

        // Read the next character
        assert_eq!(str_read.next().unwrap().unwrap(), b' ');

        // Discard the next character
        str_read.discard();

        // Read the next character
        assert_eq!(str_read.next().unwrap().unwrap(), b'"');

        // Discard again
        str_read.discard();

        // Read the next character
        assert_eq!(str_read.next().unwrap().unwrap(), b'k');
    }
}

#[cfg(test)]
mod tests_llm_16_300 {
    use super::*;

use crate::*;
    use crate::read::StrRead;

    #[test]
    fn test_ignore_str_valid() {
        let mut slice = StrRead::new("\"valid string\"");

        // Call ignore_str on a valid string
        let result = slice.ignore_str();
        assert!(result.is_ok());
        assert_eq!(slice.byte_offset(), 15); // 15 bytes for the valid string including quotes
    }

    #[test]
    fn test_ignore_str_unclosed_string() {
        let mut slice = StrRead::new("\"unclosed string");

        // Call ignore_str on a string that is unclosed
        let result = slice.ignore_str();
        assert!(result.is_err());
        // You can further check the error kind if needed
    }

    #[test]
    fn test_ignore_str_escape_sequence() {
        let mut slice = StrRead::new("\"string with escape \\\" character\"");

        // Call ignore_str on a string with escape character
        let result = slice.ignore_str();
        assert!(result.is_ok());
        assert_eq!(slice.byte_offset(), 33); // 33 bytes for the string with escape character
    }

    #[test]
    fn test_ignore_str_control_character() {
        let mut slice = StrRead::new("\"string with control char \0\"");

        // Call ignore_str on a string that contains a control character
        let result = slice.ignore_str();
        assert!(result.is_err());
        // You can further check the error kind if needed
    }
}

#[cfg(test)]
mod tests_llm_16_304 {
    use super::*;

use crate::*;
    use crate::read::{SliceRead, StrRead};
    use serde::de::Error;

    #[test]
    fn test_peek_valid_byte() {
        let input = "hello".as_bytes();
        let mut reader = SliceRead::new(input);
        assert_eq!(reader.peek().unwrap(), Some(b'h'));
        reader.next().unwrap(); // Advance the reader
        assert_eq!(reader.peek().unwrap(), Some(b'e'));
    }

    #[test]
    fn test_peek_end_of_input() {
        let input = "abc".as_bytes();
        let mut reader = SliceRead::new(input);
        reader.next().unwrap(); // Read 'a'
        reader.next().unwrap(); // Read 'b'
        reader.next().unwrap(); // Read 'c'
        assert_eq!(reader.peek().unwrap(), None); // No more bytes to peek
    }

    #[test]
    fn test_peek_with_strread() {
        let input = "test string";
        let mut reader = StrRead::new(input);
        assert_eq!(reader.peek().unwrap(), Some(b't'));
        reader.next().unwrap(); // Advance the reader
        assert_eq!(reader.peek().unwrap(), Some(b'e'));
    }

    #[test]
    fn test_peek_empty_input() {
        let input: &[u8] = b"";
        let mut reader = SliceRead::new(input);
        assert_eq!(reader.peek().unwrap(), None); // No bytes to peek
    }
}

#[cfg(test)]
mod tests_llm_16_305 {
    use super::*;

use crate::*;
    use crate::read::{StrRead, Position};

    #[test]
    fn test_peek_position() {
        let input = "Hello, world!\nThis is a test.";
        let reader = StrRead::new(input);

        // Initial position should be (1, 1)
        let position = reader.peek_position();
        assert_eq!(position.line, 1);
        assert_eq!(position.column, 1);

        // Move to next character
        let mut reader = reader;
        reader.delegate.next().unwrap(); // consumes 'H'

        // Position after consuming 'H' should be (1, 2)
        let position = reader.peek_position();
        assert_eq!(position.line, 1);
        assert_eq!(position.column, 2);

        // Move to newline
        for _ in 0..13 { // 'Hello, world!' has 13 characters
            reader.delegate.next().unwrap();
        }

        // Position after consuming till newline should be (2, 1)
        let position = reader.peek_position();
        assert_eq!(position.line, 2);
        assert_eq!(position.column, 1);
    }
}

#[cfg(test)]
mod tests_llm_16_306 {
    use super::*;

use crate::*;
    use crate::read::{StrRead, Position};

    #[test]
    fn test_position_initial() {
        let input = "Hello, world!";
        let reader = StrRead::new(input);
        let position = reader.position();
        assert_eq!(position.line, 1);
        assert_eq!(position.column, 0);
    }

    #[test]
    fn test_position_after_read() {
        let input = "Hello, world!";
        let mut reader = StrRead::new(input);
        let _ = reader.next(); // Read one byte
        let position = reader.position();
        assert_eq!(position.line, 1);
        assert_eq!(position.column, 1); // After reading 'H'
    }

    #[test]
    fn test_position_after_newline() {
        let input = "Hello,\nworld!";
        let mut reader = StrRead::new(input);
        let _ = reader.next(); // Read 'H'
        let _ = reader.next(); // Read 'e'
        let _ = reader.next(); // Read 'l'
        let _ = reader.next(); // Read 'l'
        let _ = reader.next(); // Read 'o'
        let _ = reader.next(); // Read ','
        let _ = reader.next(); // Read '\n'
        let position = reader.position();
        assert_eq!(position.line, 2);
        assert_eq!(position.column, 0); // After reading '\n'
    }
}

#[cfg(test)]
mod tests_llm_16_307 {
    use super::*;

use crate::*;
    use crate::read::{SliceRead, StrRead};

    #[test]
    fn test_set_failed() {
        let mut failed = false;
        let mut str_read = StrRead::new("test input");
        
        // Initially, `failed` should be false
        assert!(!failed);
        
        // Simulate setting failed
        str_read.set_failed(&mut failed);
        
        // After calling `set_failed`, we can check the internal state
        assert_eq!(str_read.delegate.index, 10); // The index should be at the end
        assert!(!failed); // The `failed` flag remains false as per the function logic
    }

    #[test]
    fn test_set_failed_empty_input() {
        let mut failed = false;
        let mut str_read = StrRead::new("");
        
        // Initially, `failed` should be false
        assert!(!failed);
        
        // Simulate setting failed
        str_read.set_failed(&mut failed);
        
        // After calling `set_failed` on an empty input, we check the index
        assert_eq!(str_read.delegate.index, 0); // The index should still be 0
        assert!(!failed); // The `failed` flag should still be false
    }
}

#[cfg(test)]
mod tests_llm_16_678 {
    use super::*;

use crate::*;
    use std::io::Cursor;

    #[test]
    fn test_io_read_new() {
        let input_data = b"test data\nwith multiple lines\n";
        let cursor = Cursor::new(input_data);

        let io_read = IoRead::new(cursor);

        assert_eq!(io_read.iter.line(), 1);
        assert_eq!(io_read.iter.col(), 0);
    }

    #[test]
    fn test_io_read_new_empty() {
        let input_data = b"";
        let cursor = Cursor::new(input_data);

        let io_read = IoRead::new(cursor);

        assert_eq!(io_read.iter.line(), 1);
        assert_eq!(io_read.iter.col(), 0);
    }

    #[test]
    fn test_io_read_new_single_line() {
        let input_data = b"single line";
        let cursor = Cursor::new(input_data);

        let io_read = IoRead::new(cursor);

        assert_eq!(io_read.iter.line(), 1);
        assert_eq!(io_read.iter.col(), 0);
    }
}

#[cfg(test)]
mod tests_llm_16_680 {
    use super::*;

use crate::*;
    use crate::read::SliceRead;

    #[test]
    fn test_slice_read_new_empty_slice() {
        let slice: &[u8] = &[];
        let reader = SliceRead::new(slice);
        assert_eq!(reader.index, 0);
        assert_eq!(reader.slice.len(), 0);
    }

    #[test]
    fn test_slice_read_new_non_empty_slice() {
        let slice: &[u8] = b"{\"key\": \"value\"}";
        let reader = SliceRead::new(slice);
        assert_eq!(reader.index, 0);
        assert_eq!(reader.slice.len(), slice.len());
    }

    #[test]
    fn test_slice_read_new_with_large_slice() {
        let slice: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        let reader = SliceRead::new(slice);
        assert_eq!(reader.index, 0);
        assert_eq!(reader.slice.len(), slice.len());
    }
}

#[cfg(test)]
mod tests_llm_16_682 {
    use super::*;

use crate::*;
    use crate::read::SliceRead;

    #[test]
    fn test_position_of_index() {
        let data: &[u8] = b"line1\nline2\nline3";
        let reader = SliceRead::new(data);
        
        let pos_0 = reader.position_of_index(0);
        assert_eq!(pos_0.line, 1);
        assert_eq!(pos_0.column, 0);
        
        let pos_5 = reader.position_of_index(5);
        assert_eq!(pos_5.line, 1);
        assert_eq!(pos_5.column, 5);
        
        let pos_6 = reader.position_of_index(6);
        assert_eq!(pos_6.line, 2);
        assert_eq!(pos_6.column, 0);
        
        let pos_12 = reader.position_of_index(12);
        assert_eq!(pos_12.line, 3);
        assert_eq!(pos_12.column, 5);
    }

    #[test]
    fn test_position_of_index_with_newline() {
        let data: &[u8] = b"\nline1\nline2";
        let reader = SliceRead::new(data);

        let pos_1 = reader.position_of_index(1);
        assert_eq!(pos_1.line, 1);
        assert_eq!(pos_1.column, 0);

        let pos_2 = reader.position_of_index(2);
        assert_eq!(pos_2.line, 1);
        assert_eq!(pos_2.column, 1);

        let pos_3 = reader.position_of_index(3);
        assert_eq!(pos_3.line, 2);
        assert_eq!(pos_3.column, 0);
    }
}

#[cfg(test)]
mod tests_llm_16_683 {
    use super::*;

use crate::*;
    use crate::read::SliceRead;

    #[test]
    fn test_skip_to_escape_no_escape() {
        let slice = b"Hello, world!";
        let mut reader = SliceRead::new(slice);
        reader.skip_to_escape(false);
        assert_eq!(reader.index, slice.len());
    }

    #[test]
    fn test_skip_to_escape_with_escape() {
        let slice = b"Hello, world!\"This is a test.";
        let mut reader = SliceRead::new(slice);
        reader.skip_to_escape(false);
        assert_eq!(reader.index, slice.len());
        reader.index = 0; // reset index
        reader.skip_to_escape(true);
        assert_eq!(reader.index, 8); // should skip to first quote
    }

    #[test]
    fn test_skip_to_escape_forbid_control() {
        let slice = b"Hello\x01, world!";
        let mut reader = SliceRead::new(slice);
        reader.skip_to_escape(true);
        assert_eq!(reader.index, 7); // should skip to first control character
    }

    #[test]
    fn test_skip_to_escape_multiple_escapes() {
        let slice = b"\"Hello\\\" world\\\"!\"";
        let mut reader = SliceRead::new(slice);
        reader.skip_to_escape(false);
        assert_eq!(reader.index, 14); // should skip to the last quote
    }

    #[test]
    fn test_skip_to_escape_empty_slice() {
        let slice: &[u8] = &[];
        let mut reader = SliceRead::new(slice);
        reader.skip_to_escape(true);
        assert_eq!(reader.index, 0); // should remain at index 0
    }

    #[test]
    fn test_skip_to_escape_consecutive_escapes() {
        let slice = b"\\\"Hello\\\" world!";
        let mut reader = SliceRead::new(slice);
        reader.skip_to_escape(false);
        assert_eq!(reader.index, 2); // should skip the first escape
    }
}

#[cfg(test)]
mod tests_llm_16_685 {
    use super::*;

use crate::*;
    use crate::read::StrRead;

    #[test]
    fn test_str_read_new() {
        let json_str = r#"{ "key": "value" }"#;
        let str_read = StrRead::new(json_str);

        // Check if the underlying slice is correct
        assert_eq!(str_read.delegate.slice, json_str.as_bytes());
        assert_eq!(str_read.delegate.index, 0);
    }

    #[test]
    fn test_str_read_new_empty() {
        let json_str = "";
        let str_read = StrRead::new(json_str);

        // Check if the underlying slice is correct for an empty string
        assert_eq!(str_read.delegate.slice, json_str.as_bytes());
        assert_eq!(str_read.delegate.index, 0);
    }
}

#[cfg(test)]
mod tests_llm_16_687 {
    use super::*;

use crate::*;

    #[test]
    fn test_build_hex_table() {
        let expected: [i16; 256] = [
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1, -1, -1, -1, -1, -1, 
            10, 11, 12, 13, 14, 15, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 
            10, 11, 12, 13, 14, 15, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 
            10, 11, 12, 13, 14, 15, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 
        ]; // should be filled with expected values for the test
        let shift = 0; // adjust as necessary
        let result = build_hex_table(shift);
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod tests_llm_16_688 {
    use super::*;

use crate::*;

    #[test]
    fn test_decode_four_hex_digits_valid() {
        assert_eq!(decode_four_hex_digits(0x30, 0x30, 0x30, 0x30), Some(0x0030)); // '0'
        assert_eq!(decode_four_hex_digits(0x41, 0x41, 0x41, 0x41), Some(0x4141)); // 'A'
        assert_eq!(decode_four_hex_digits(0x7A, 0x7A, 0x7A, 0x7A), Some(0x007A)); // 'z'
    }

    #[test]
    fn test_decode_four_hex_digits_invalid() {
        assert_eq!(decode_four_hex_digits(0xFF, 0xFF, 0xFF, 0xFF), None); // Invalid hex
    }

    #[test]
    fn test_decode_four_hex_digits_boundary() {
        assert_eq!(decode_four_hex_digits(0x00, 0x00, 0x00, 0x00), Some(0x0000)); // '\x00'
        assert_eq!(decode_four_hex_digits(0x0F, 0x0F, 0x0F, 0x0F), Some(0x0FFF)); // Valid upper bound
    }
}

#[cfg(test)]
mod tests_llm_16_691 {
    use super::*;

use crate::*;
    use std::io::Cursor;

    #[test]
    fn test_ignore_escape_valid_characters() {
        let input = b"\\\"\\'\\\\\\/\\b\\f\\n\\r\\t\\u003A";
        let mut reader = IoRead::new(Cursor::new(input));

        // Test correct handling of valid escape characters
        assert!(ignore_escape(&mut reader).is_ok()); // "
        assert!(ignore_escape(&mut reader).is_ok()); // '
        assert!(ignore_escape(&mut reader).is_ok()); // \
        assert!(ignore_escape(&mut reader).is_ok()); // /
        assert!(ignore_escape(&mut reader).is_ok()); // b
        assert!(ignore_escape(&mut reader).is_ok()); // f
        assert!(ignore_escape(&mut reader).is_ok()); // n
        assert!(ignore_escape(&mut reader).is_ok()); // r
        assert!(ignore_escape(&mut reader).is_ok()); // t
        assert!(ignore_escape(&mut reader).is_ok()); // u
    }

    #[test]
    fn test_ignore_escape_invalid_character() {
        let input = b"\\k";
        let mut reader = IoRead::new(Cursor::new(input));

        // Test handling of invalid escape character
        let result = ignore_escape(&mut reader);
        assert!(result.is_err());
    }

    #[test]
    fn test_ignore_escape_hex_sequence() {
        let input = b"\\u003A";
        let mut reader = IoRead::new(Cursor::new(input));

        // Test handling of valid hex escape sequence
        assert!(ignore_escape(&mut reader).is_ok());
    }
}

#[cfg(test)]
mod tests_llm_16_692 {
    use crate::read::is_escape;

    #[test]
    fn test_is_escape() {
        assert!(is_escape(b'"', false));
        assert!(is_escape(b'\\', false));
        assert!(is_escape(b'\n', true));
        assert!(is_escape(b'\t', true));
        assert!(!is_escape(b'a', false));
        assert!(!is_escape(b'\\', true));
        assert!(!is_escape(b'\x20', false));
        assert!(!is_escape(b' ', false));
    }
}

#[cfg(test)]
mod tests_llm_16_694 {
    use super::*;

use crate::*;
    use crate::read::IoRead;
    use std::io::Cursor;

    #[test]
    fn test_parse_escape_valid_sequences() {
        let data = b"\\\"\\'\\\\\\b\\f\\n\\r\\t\\/";
        let mut scratch = Vec::new();
        let mut reader = IoRead::new(Cursor::new(data));

        // test each escape sequence
        for byte in data.iter() {
            let result = parse_escape(&mut reader, false, &mut scratch);
            assert!(result.is_ok());
        }

        assert_eq!(scratch, b"\"'\\\x08\x0c\x0a\x0d\x09\x2f");
    }

    #[test]
    fn test_parse_escape_invalid_sequence() {
        let data = b"\\x";
        let mut scratch = Vec::new();
        let mut reader = IoRead::new(Cursor::new(data));

        // Expecting an error due to invalid escape sequence
        let result = parse_escape(&mut reader, false, &mut scratch);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_escape_unicode_sequence() {
        let data = b"\\u003A"; // represents a colon `:`
        let mut scratch = Vec::new();
        let mut reader = IoRead::new(Cursor::new(data));

        // parse escape sequence
        let result = parse_escape(&mut reader, false, &mut scratch);
        assert!(result.is_ok());
        assert_eq!(scratch, b":");
    }
}

use crate::{engine::Engine, DecodeError, DecodeSliceError, PAD_BYTE};
use std::{cmp, fmt, io};
pub(crate) const BUF_SIZE: usize = 1024;
const BASE64_CHUNK_SIZE: usize = 4;
const DECODED_CHUNK_SIZE: usize = 3;
pub trait Engine: Send + Sync {
    type Config: Config;
    type DecodeEstimate: DecodeEstimate;
    fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize;
    fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate;
    fn internal_decode(
        &self,
        input: &[u8],
        output: &mut [u8],
        decode_estimate: Self::DecodeEstimate,
    ) -> Result<DecodeMetadata, DecodeSliceError>;
    fn config(&self) -> &Self::Config;
    #[cfg(any(feature = "alloc", test))]
    #[inline]
    fn encode<T: AsRef<[u8]>>(&self, input: T) -> String;
    #[cfg(any(feature = "alloc", test))]
    #[inline]
    fn encode_string<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut String);
    #[cfg_attr(feature = "alloc", doc = "```")]
    #[cfg_attr(not(feature = "alloc"), doc = "```ignore")]
    #[inline]
    fn encode_slice<T: AsRef<[u8]>>(
        &self,
        input: T,
        output_buf: &mut [u8],
    ) -> Result<usize, EncodeSliceError>;
    #[cfg(any(feature = "alloc", test))]
    #[inline]
    fn decode<T: AsRef<[u8]>>(&self, input: T) -> Result<Vec<u8>, DecodeError>;
    #[cfg(any(feature = "alloc", test))]
    #[inline]
    fn decode_vec<T: AsRef<[u8]>>(
        &self,
        input: T,
        buffer: &mut Vec<u8>,
    ) -> Result<(), DecodeError>;
    #[inline]
    fn decode_slice<T: AsRef<[u8]>>(
        &self,
        input: T,
        output: &mut [u8],
    ) -> Result<usize, DecodeSliceError>;
    #[inline]
    fn decode_slice_unchecked<T: AsRef<[u8]>>(
        &self,
        input: T,
        output: &mut [u8],
    ) -> Result<usize, DecodeError>;
}
pub struct DecoderReader<'e, E: Engine, R: io::Read> {
    engine: &'e E,
    /// Where b64 data is read from
    inner: R,
    /// Holds b64 data read from the delegate reader.
    b64_buffer: [u8; BUF_SIZE],
    /// The start of the pending buffered data in `b64_buffer`.
    b64_offset: usize,
    /// The amount of buffered b64 data after `b64_offset` in `b64_len`.
    b64_len: usize,
    /// Since the caller may provide us with a buffer of size 1 or 2 that's too small to copy a
    /// decoded chunk in to, we have to be able to hang on to a few decoded bytes.
    /// Technically we only need to hold 2 bytes, but then we'd need a separate temporary buffer to
    /// decode 3 bytes into and then juggle copying one byte into the provided read buf and the rest
    /// into here, which seems like a lot of complexity for 1 extra byte of storage.
    decoded_chunk_buffer: [u8; DECODED_CHUNK_SIZE],
    /// Index of start of decoded data in `decoded_chunk_buffer`
    decoded_offset: usize,
    /// Length of decoded data after `decoded_offset` in `decoded_chunk_buffer`
    decoded_len: usize,
    /// Input length consumed so far.
    /// Used to provide accurate offsets in errors
    input_consumed_len: usize,
    /// offset of previously seen padding, if any
    padding_offset: Option<usize>,
}
#[derive(PartialEq, Eq, Debug)]
pub struct DecodeMetadata {
    /// Number of decoded bytes output
    pub(crate) decoded_len: usize,
    /// Offset of the first padding byte in the input, if any
    pub(crate) padding_offset: Option<usize>,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeError {
    /// An invalid byte was found in the input. The offset and offending byte are provided.
    ///
    /// Padding characters (`=`) interspersed in the encoded form are invalid, as they may only
    /// be present as the last 0-2 bytes of input.
    ///
    /// This error may also indicate that extraneous trailing input bytes are present, causing
    /// otherwise valid padding to no longer be the last bytes of input.
    InvalidByte(usize, u8),
    /// The length of the input, as measured in valid base64 symbols, is invalid.
    /// There must be 2-4 symbols in the last input quad.
    InvalidLength(usize),
    /// The last non-padding input symbol's encoded 6 bits have nonzero bits that will be discarded.
    /// This is indicative of corrupted or truncated Base64.
    /// Unlike [`DecodeError::InvalidByte`], which reports symbols that aren't in the alphabet,
    /// this error is for symbols that are in the alphabet but represent nonsensical encodings.
    InvalidLastSymbol(usize, u8),
    /// The nature of the padding was not as configured: absent or incorrect when it must be
    /// canonical, or present when it must be absent, etc.
    InvalidPadding,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeSliceError {
    /// A [`DecodeError`] occurred
    DecodeError(DecodeError),
    /// The provided slice is too small.
    OutputSliceTooSmall,
}
impl<'e, E: Engine, R: io::Read> DecoderReader<'e, E, R> {
    pub fn new(reader: R, engine: &'e E) -> Self {
        DecoderReader {
            engine,
            inner: reader,
            b64_buffer: [0; BUF_SIZE],
            b64_offset: 0,
            b64_len: 0,
            decoded_chunk_buffer: [0; DECODED_CHUNK_SIZE],
            decoded_offset: 0,
            decoded_len: 0,
            input_consumed_len: 0,
            padding_offset: None,
        }
    }
    fn flush_decoded_buf(&mut self, buf: &mut [u8]) -> io::Result<usize> {}
    fn read_from_delegate(&mut self) -> io::Result<usize> {}
    fn decode_to_buf(
        &mut self,
        b64_len_to_decode: usize,
        buf: &mut [u8],
    ) -> io::Result<usize> {
        debug_assert!(self.b64_len >= b64_len_to_decode);
        debug_assert!(self.b64_offset + self.b64_len <= BUF_SIZE);
        debug_assert!(! buf.is_empty());
        let b64_to_decode = &self
            .b64_buffer[self.b64_offset..self.b64_offset + b64_len_to_decode];
        let decode_metadata = self
            .engine
            .internal_decode(
                b64_to_decode,
                buf,
                self.engine.internal_decoded_len_estimate(b64_len_to_decode),
            )
            .map_err(|dse| match dse {
                DecodeSliceError::DecodeError(de) => {
                    match de {
                        DecodeError::InvalidByte(offset, byte) => {
                            match (byte, self.padding_offset) {
                                (PAD_BYTE, Some(first_pad_offset)) => {
                                    DecodeError::InvalidByte(first_pad_offset, PAD_BYTE)
                                }
                                _ => {
                                    DecodeError::InvalidByte(
                                        self.input_consumed_len + offset,
                                        byte,
                                    )
                                }
                            }
                        }
                        DecodeError::InvalidLength(len) => {
                            DecodeError::InvalidLength(self.input_consumed_len + len)
                        }
                        DecodeError::InvalidLastSymbol(offset, byte) => {
                            DecodeError::InvalidLastSymbol(
                                self.input_consumed_len + offset,
                                byte,
                            )
                        }
                        DecodeError::InvalidPadding => DecodeError::InvalidPadding,
                    }
                }
                DecodeSliceError::OutputSliceTooSmall => {
                    unreachable!("buf is sized correctly in calling code")
                }
            })
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        if let Some(offset) = self.padding_offset {
            if decode_metadata.decoded_len > 0 {
                return Err(
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        DecodeError::InvalidByte(offset, PAD_BYTE),
                    ),
                );
            }
        }
        self.padding_offset = self
            .padding_offset
            .or(
                decode_metadata
                    .padding_offset
                    .map(|offset| self.input_consumed_len + offset),
            );
        self.input_consumed_len += b64_len_to_decode;
        self.b64_offset += b64_len_to_decode;
        self.b64_len -= b64_len_to_decode;
        debug_assert!(self.b64_offset + self.b64_len <= BUF_SIZE);
        Ok(decode_metadata.decoded_len)
    }
    pub fn into_inner(self) -> R {}
}

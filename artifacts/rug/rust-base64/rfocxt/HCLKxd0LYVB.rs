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
impl<'e, E: Engine, R: io::Read> fmt::Debug for DecoderReader<'e, E, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("DecoderReader")
            .field("b64_offset", &self.b64_offset)
            .field("b64_len", &self.b64_len)
            .field("decoded_chunk_buffer", &self.decoded_chunk_buffer)
            .field("decoded_offset", &self.decoded_offset)
            .field("decoded_len", &self.decoded_len)
            .field("input_consumed_len", &self.input_consumed_len)
            .field("padding_offset", &self.padding_offset)
            .finish()
    }
}

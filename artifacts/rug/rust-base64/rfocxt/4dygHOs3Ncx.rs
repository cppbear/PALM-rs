use crate::{encode::add_padding, engine::{Config, Engine}};
#[cfg(any(feature = "alloc", test))]
use alloc::string::String;
#[cfg(any(feature = "alloc", test))]
use core::str;
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
pub struct ChunkedEncoder<'e, E: Engine + ?Sized> {
    engine: &'e E,
}
impl<'e, E: Engine + ?Sized> ChunkedEncoder<'e, E> {
    pub fn new(engine: &'e E) -> ChunkedEncoder<'e, E> {}
    pub fn encode<S: Sink>(&self, bytes: &[u8], sink: &mut S) -> Result<(), S::Error> {
        const BUF_SIZE: usize = 1024;
        const CHUNK_SIZE: usize = BUF_SIZE / 4 * 3;
        let mut buf = [0; BUF_SIZE];
        for chunk in bytes.chunks(CHUNK_SIZE) {
            let mut len = self.engine.internal_encode(chunk, &mut buf);
            if chunk.len() != CHUNK_SIZE && self.engine.config().encode_padding() {
                len += add_padding(len, &mut buf[len..]);
            }
            sink.write_encoded_bytes(&buf[..len])?;
        }
        Ok(())
    }
}
pub(crate) fn add_padding(unpadded_output_len: usize, output: &mut [u8]) -> usize {
    let pad_bytes = (4 - (unpadded_output_len % 4)) % 4;
    #[allow(clippy::needless_range_loop)]
    for i in 0..pad_bytes {
        output[i] = PAD_BYTE;
    }
    pad_bytes
}

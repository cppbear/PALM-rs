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
    pub fn new(engine: &'e E) -> ChunkedEncoder<'e, E> {
        ChunkedEncoder { engine }
    }
    pub fn encode<S: Sink>(&self, bytes: &[u8], sink: &mut S) -> Result<(), S::Error> {}
}

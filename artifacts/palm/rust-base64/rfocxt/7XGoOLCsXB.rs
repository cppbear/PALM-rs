use crate::engine::Engine;
use std::{cmp, fmt, io, io::{ErrorKind, Result}};
pub(crate) const BUF_SIZE: usize = 1024;
const MAX_INPUT_LEN: usize = BUF_SIZE / 4 * 3;
const MIN_ENCODE_CHUNK_SIZE: usize = 3;
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
pub struct EncoderWriter<'e, E: Engine, W: io::Write> {
    engine: &'e E,
    /// Where encoded data is written to. It's an Option as it's None immediately before Drop is
    /// called so that `finish()` can return the underlying writer. None implies that `finish()` has
    /// been called successfully.
    delegate: Option<W>,
    /// Holds a partial chunk, if any, after the last `write()`, so that we may then fill the chunk
    /// with the next `write()`, encode it, then proceed with the rest of the input normally.
    extra_input: [u8; MIN_ENCODE_CHUNK_SIZE],
    /// How much of `extra` is occupied, in `[0, MIN_ENCODE_CHUNK_SIZE]`.
    extra_input_occupied_len: usize,
    /// Buffer to encode into. May hold leftover encoded bytes from a previous write call that the underlying writer
    /// did not write last time.
    output: [u8; BUF_SIZE],
    /// How much of `output` is occupied with encoded data that couldn't be written last time
    output_occupied_len: usize,
    /// panic safety: don't write again in destructor if writer panicked while we were writing to it
    panicked: bool,
}
impl<'e, E: Engine, W: io::Write> EncoderWriter<'e, E, W> {
    pub fn new(delegate: W, engine: &'e E) -> EncoderWriter<'e, E, W> {}
    pub fn finish(&mut self) -> Result<W> {}
    fn write_final_leftovers(&mut self) -> Result<()> {}
    fn write_to_delegate(&mut self, current_output_len: usize) -> Result<()> {
        self.panicked = true;
        let res = self
            .delegate
            .as_mut()
            .expect("Writer must be present")
            .write(&self.output[..current_output_len]);
        self.panicked = false;
        res.map(|consumed| {
            debug_assert!(consumed <= current_output_len);
            if consumed < current_output_len {
                self.output_occupied_len = current_output_len
                    .checked_sub(consumed)
                    .unwrap();
                self.output.rotate_left(consumed);
            } else {
                self.output_occupied_len = 0;
            }
        })
    }
    fn write_all_encoded_output(&mut self) -> Result<()> {}
    pub fn into_inner(mut self) -> W {}
}

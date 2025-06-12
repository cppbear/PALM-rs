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
impl<'e, E: Engine, W: io::Write> io::Write for EncoderWriter<'e, E, W> {
    fn write(&mut self, input: &[u8]) -> Result<usize> {
        assert!(self.delegate.is_some(), "Cannot write more after calling finish()");
        if input.is_empty() {
            return Ok(0);
        }
        if self.output_occupied_len > 0 {
            let current_len = self.output_occupied_len;
            return self.write_to_delegate(current_len).map(|()| 0);
        }
        debug_assert_eq!(0, self.output_occupied_len);
        let mut extra_input_read_len = 0;
        let mut input = input;
        let orig_extra_len = self.extra_input_occupied_len;
        let mut encoded_size = 0;
        let mut max_input_len = MAX_INPUT_LEN;
        if self.extra_input_occupied_len > 0 {
            debug_assert!(self.extra_input_occupied_len < 3);
            if input.len() + self.extra_input_occupied_len >= MIN_ENCODE_CHUNK_SIZE {
                extra_input_read_len = MIN_ENCODE_CHUNK_SIZE
                    - self.extra_input_occupied_len;
                debug_assert!(extra_input_read_len > 0);
                self.extra_input[self.extra_input_occupied_len..MIN_ENCODE_CHUNK_SIZE]
                    .copy_from_slice(&input[0..extra_input_read_len]);
                let len = self
                    .engine
                    .internal_encode(
                        &self.extra_input[0..MIN_ENCODE_CHUNK_SIZE],
                        &mut self.output[..],
                    );
                debug_assert_eq!(4, len);
                input = &input[extra_input_read_len..];
                self.extra_input_occupied_len = 0;
                encoded_size = 4;
                max_input_len = MAX_INPUT_LEN - MIN_ENCODE_CHUNK_SIZE;
            } else {
                debug_assert_eq!(1, input.len());
                debug_assert_eq!(1, self.extra_input_occupied_len);
                self.extra_input[self.extra_input_occupied_len] = input[0];
                self.extra_input_occupied_len += 1;
                return Ok(1);
            };
        } else if input.len() < MIN_ENCODE_CHUNK_SIZE {
            self.extra_input[0..input.len()].copy_from_slice(input);
            self.extra_input_occupied_len = input.len();
            return Ok(input.len());
        }
        debug_assert!(encoded_size == 0 || encoded_size == 4);
        debug_assert!(
            MAX_INPUT_LEN == max_input_len || MAX_INPUT_LEN == max_input_len +
            MIN_ENCODE_CHUNK_SIZE
        );
        let input_complete_chunks_len = input.len()
            - (input.len() % MIN_ENCODE_CHUNK_SIZE);
        let input_chunks_to_encode_len = cmp::min(
            input_complete_chunks_len,
            max_input_len,
        );
        debug_assert_eq!(0, max_input_len % MIN_ENCODE_CHUNK_SIZE);
        debug_assert_eq!(0, input_chunks_to_encode_len % MIN_ENCODE_CHUNK_SIZE);
        encoded_size
            += self
                .engine
                .internal_encode(
                    &input[..(input_chunks_to_encode_len)],
                    &mut self.output[encoded_size..],
                );
        self.write_to_delegate(encoded_size)
            .map(|()| extra_input_read_len + input_chunks_to_encode_len)
            .map_err(|e| {
                self.extra_input_occupied_len = orig_extra_len;
                e
            })
    }
    fn flush(&mut self) -> Result<()> {}
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

use crate::{
    alphabet, alphabet::Alphabet, engine::{Config, DecodeMetadata, DecodePaddingMode},
    DecodeSliceError,
};
use core::convert::TryInto;
pub use decode::GeneralPurposeEstimate;
pub(crate) const INVALID_VALUE: u8 = 255;
pub const STANDARD: GeneralPurpose = GeneralPurpose::new(&alphabet::STANDARD, PAD);
pub const STANDARD_PAD_INDIFFERENT: GeneralPurpose = GeneralPurpose::new(
    &alphabet::STANDARD,
    PAD_INDIFFERENT,
);
pub const STANDARD_NO_PAD: GeneralPurpose = GeneralPurpose::new(
    &alphabet::STANDARD,
    NO_PAD,
);
pub const STANDARD_NO_PAD_INDIFFERENT: GeneralPurpose = GeneralPurpose::new(
    &alphabet::STANDARD,
    NO_PAD_INDIFFERENT,
);
pub const URL_SAFE: GeneralPurpose = GeneralPurpose::new(&alphabet::URL_SAFE, PAD);
pub const URL_SAFE_PAD_INDIFFERENT: GeneralPurpose = GeneralPurpose::new(
    &alphabet::URL_SAFE,
    PAD_INDIFFERENT,
);
pub const URL_SAFE_NO_PAD: GeneralPurpose = GeneralPurpose::new(
    &alphabet::URL_SAFE,
    NO_PAD,
);
pub const URL_SAFE_NO_PAD_INDIFFERENT: GeneralPurpose = GeneralPurpose::new(
    &alphabet::URL_SAFE,
    NO_PAD_INDIFFERENT,
);
pub const PAD: GeneralPurposeConfig = GeneralPurposeConfig::new();
pub const PAD_INDIFFERENT: GeneralPurposeConfig = GeneralPurposeConfig::new()
    .with_encode_padding(true)
    .with_decode_padding_mode(DecodePaddingMode::Indifferent);
pub const NO_PAD: GeneralPurposeConfig = GeneralPurposeConfig::new()
    .with_encode_padding(false)
    .with_decode_padding_mode(DecodePaddingMode::RequireNone);
pub const NO_PAD_INDIFFERENT: GeneralPurposeConfig = GeneralPurposeConfig::new()
    .with_encode_padding(false)
    .with_decode_padding_mode(DecodePaddingMode::Indifferent);
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
    fn encode<T: AsRef<[u8]>>(&self, input: T) -> String {
        fn inner<E>(engine: &E, input_bytes: &[u8]) -> String
        where
            E: Engine + ?Sized,
        {
            let encoded_size = encoded_len(
                    input_bytes.len(),
                    engine.config().encode_padding(),
                )
                .expect("integer overflow when calculating buffer size");
            let mut buf = vec![0; encoded_size];
            encode_with_padding(input_bytes, &mut buf[..], engine, encoded_size);
            String::from_utf8(buf).expect("Invalid UTF8")
        }
        inner(self, input.as_ref())
    }
    #[cfg(any(feature = "alloc", test))]
    #[inline]
    fn encode_string<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut String) {
        fn inner<E>(engine: &E, input_bytes: &[u8], output_buf: &mut String)
        where
            E: Engine + ?Sized,
        {
            let mut sink = chunked_encoder::StringSink::new(output_buf);
            chunked_encoder::ChunkedEncoder::new(engine)
                .encode(input_bytes, &mut sink)
                .expect("Writing to a String shouldn't fail");
        }
        inner(self, input.as_ref(), output_buf);
    }
    #[cfg_attr(feature = "alloc", doc = "```")]
    #[cfg_attr(not(feature = "alloc"), doc = "```ignore")]
    #[inline]
    fn encode_slice<T: AsRef<[u8]>>(
        &self,
        input: T,
        output_buf: &mut [u8],
    ) -> Result<usize, EncodeSliceError> {
        fn inner<E>(
            engine: &E,
            input_bytes: &[u8],
            output_buf: &mut [u8],
        ) -> Result<usize, EncodeSliceError>
        where
            E: Engine + ?Sized,
        {
            let encoded_size = encoded_len(
                    input_bytes.len(),
                    engine.config().encode_padding(),
                )
                .expect("usize overflow when calculating buffer size");
            if output_buf.len() < encoded_size {
                return Err(EncodeSliceError::OutputSliceTooSmall);
            }
            let b64_output = &mut output_buf[0..encoded_size];
            encode_with_padding(input_bytes, b64_output, engine, encoded_size);
            Ok(encoded_size)
        }
        inner(self, input.as_ref(), output_buf)
    }
    #[cfg(any(feature = "alloc", test))]
    #[inline]
    fn decode<T: AsRef<[u8]>>(&self, input: T) -> Result<Vec<u8>, DecodeError> {
        fn inner<E>(engine: &E, input_bytes: &[u8]) -> Result<Vec<u8>, DecodeError>
        where
            E: Engine + ?Sized,
        {
            let estimate = engine.internal_decoded_len_estimate(input_bytes.len());
            let mut buffer = vec![0; estimate.decoded_len_estimate()];
            let bytes_written = engine
                .internal_decode(input_bytes, &mut buffer, estimate)
                .map_err(|e| match e {
                    DecodeSliceError::DecodeError(e) => e,
                    DecodeSliceError::OutputSliceTooSmall => {
                        unreachable!("Vec is sized conservatively")
                    }
                })?
                .decoded_len;
            buffer.truncate(bytes_written);
            Ok(buffer)
        }
        inner(self, input.as_ref())
    }
    #[cfg(any(feature = "alloc", test))]
    #[inline]
    fn decode_vec<T: AsRef<[u8]>>(
        &self,
        input: T,
        buffer: &mut Vec<u8>,
    ) -> Result<(), DecodeError> {
        fn inner<E>(
            engine: &E,
            input_bytes: &[u8],
            buffer: &mut Vec<u8>,
        ) -> Result<(), DecodeError>
        where
            E: Engine + ?Sized,
        {
            let starting_output_len = buffer.len();
            let estimate = engine.internal_decoded_len_estimate(input_bytes.len());
            let total_len_estimate = estimate
                .decoded_len_estimate()
                .checked_add(starting_output_len)
                .expect("Overflow when calculating output buffer length");
            buffer.resize(total_len_estimate, 0);
            let buffer_slice = &mut buffer.as_mut_slice()[starting_output_len..];
            let bytes_written = engine
                .internal_decode(input_bytes, buffer_slice, estimate)
                .map_err(|e| match e {
                    DecodeSliceError::DecodeError(e) => e,
                    DecodeSliceError::OutputSliceTooSmall => {
                        unreachable!("Vec is sized conservatively")
                    }
                })?
                .decoded_len;
            buffer.truncate(starting_output_len + bytes_written);
            Ok(())
        }
        inner(self, input.as_ref(), buffer)
    }
    #[inline]
    fn decode_slice<T: AsRef<[u8]>>(
        &self,
        input: T,
        output: &mut [u8],
    ) -> Result<usize, DecodeSliceError> {
        fn inner<E>(
            engine: &E,
            input_bytes: &[u8],
            output: &mut [u8],
        ) -> Result<usize, DecodeSliceError>
        where
            E: Engine + ?Sized,
        {
            engine
                .internal_decode(
                    input_bytes,
                    output,
                    engine.internal_decoded_len_estimate(input_bytes.len()),
                )
                .map(|dm| dm.decoded_len)
        }
        inner(self, input.as_ref(), output)
    }
    #[inline]
    fn decode_slice_unchecked<T: AsRef<[u8]>>(
        &self,
        input: T,
        output: &mut [u8],
    ) -> Result<usize, DecodeError> {
        fn inner<E>(
            engine: &E,
            input_bytes: &[u8],
            output: &mut [u8],
        ) -> Result<usize, DecodeError>
        where
            E: Engine + ?Sized,
        {
            engine
                .internal_decode(
                    input_bytes,
                    output,
                    engine.internal_decoded_len_estimate(input_bytes.len()),
                )
                .map(|dm| dm.decoded_len)
                .map_err(|e| match e {
                    DecodeSliceError::DecodeError(e) => e,
                    DecodeSliceError::OutputSliceTooSmall => {
                        panic!("Output slice is too small")
                    }
                })
        }
        inner(self, input.as_ref(), output)
    }
}
#[derive(Debug, Clone)]
pub struct GeneralPurpose {
    encode_table: [u8; 64],
    decode_table: [u8; 256],
    config: GeneralPurposeConfig,
}
#[derive(Clone, Copy, Debug)]
pub struct GeneralPurposeConfig {
    encode_padding: bool,
    decode_allow_trailing_bits: bool,
    decode_padding_mode: DecodePaddingMode,
}
pub struct GeneralPurposeEstimate {
    /// input len % 4
    rem: usize,
    conservative_decoded_len: usize,
}
#[derive(PartialEq, Eq, Debug)]
pub struct DecodeMetadata {
    /// Number of decoded bytes output
    pub(crate) decoded_len: usize,
    /// Offset of the first padding byte in the input, if any
    pub(crate) padding_offset: Option<usize>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecodePaddingMode {
    /// Canonical padding is allowed, but any fewer padding bytes than that is also allowed.
    Indifferent,
    /// Padding must be canonical (0, 1, or 2 `=` as needed to produce a 4 byte suffix).
    RequireCanonical,
    /// Padding must be absent -- for when you want predictable padding, without any wasted bytes.
    RequireNone,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecodeSliceError {
    /// A [`DecodeError`] occurred
    DecodeError(DecodeError),
    /// The provided slice is too small.
    OutputSliceTooSmall,
}
impl super::Engine for GeneralPurpose {
    type Config = GeneralPurposeConfig;
    type DecodeEstimate = GeneralPurposeEstimate;
    fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {}
    fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
        GeneralPurposeEstimate::new(input_len)
    }
    fn internal_decode(
        &self,
        input: &[u8],
        output: &mut [u8],
        estimate: Self::DecodeEstimate,
    ) -> Result<DecodeMetadata, DecodeSliceError> {
        decode::decode_helper(
            input,
            &estimate,
            output,
            &self.decode_table,
            self.config.decode_allow_trailing_bits,
            self.config.decode_padding_mode,
        )
    }
    fn config(&self) -> &Self::Config {
        &self.config
    }
}
#[inline]
pub(crate) fn decode_helper(
    input: &[u8],
    estimate: &GeneralPurposeEstimate,
    output: &mut [u8],
    decode_table: &[u8; 256],
    decode_allow_trailing_bits: bool,
    padding_mode: DecodePaddingMode,
) -> Result<DecodeMetadata, DecodeSliceError> {
    let input_complete_nonterminal_quads_len = complete_quads_len(
        input,
        estimate.rem,
        output.len(),
        decode_table,
    )?;
    const UNROLLED_INPUT_CHUNK_SIZE: usize = 32;
    const UNROLLED_OUTPUT_CHUNK_SIZE: usize = UNROLLED_INPUT_CHUNK_SIZE / 4 * 3;
    let input_complete_quads_after_unrolled_chunks_len = input_complete_nonterminal_quads_len
        % UNROLLED_INPUT_CHUNK_SIZE;
    let input_unrolled_loop_len = input_complete_nonterminal_quads_len
        - input_complete_quads_after_unrolled_chunks_len;
    for (chunk_index, chunk) in input[..input_unrolled_loop_len]
        .chunks_exact(UNROLLED_INPUT_CHUNK_SIZE)
        .enumerate()
    {
        let input_index = chunk_index * UNROLLED_INPUT_CHUNK_SIZE;
        let chunk_output = &mut output[chunk_index
            * UNROLLED_OUTPUT_CHUNK_SIZE..(chunk_index + 1)
            * UNROLLED_OUTPUT_CHUNK_SIZE];
        decode_chunk_8(
            &chunk[0..8],
            input_index,
            decode_table,
            &mut chunk_output[0..6],
        )?;
        decode_chunk_8(
            &chunk[8..16],
            input_index + 8,
            decode_table,
            &mut chunk_output[6..12],
        )?;
        decode_chunk_8(
            &chunk[16..24],
            input_index + 16,
            decode_table,
            &mut chunk_output[12..18],
        )?;
        decode_chunk_8(
            &chunk[24..32],
            input_index + 24,
            decode_table,
            &mut chunk_output[18..24],
        )?;
    }
    let output_unrolled_loop_len = input_unrolled_loop_len / 4 * 3;
    let output_complete_quad_len = input_complete_nonterminal_quads_len / 4 * 3;
    {
        let output_after_unroll = &mut output[output_unrolled_loop_len..output_complete_quad_len];
        for (chunk_index, chunk) in input[input_unrolled_loop_len..input_complete_nonterminal_quads_len]
            .chunks_exact(4)
            .enumerate()
        {
            let chunk_output = &mut output_after_unroll[chunk_index
                * 3..chunk_index * 3 + 3];
            decode_chunk_4(
                chunk,
                input_unrolled_loop_len + chunk_index * 4,
                decode_table,
                chunk_output,
            )?;
        }
    }
    super::decode_suffix::decode_suffix(
        input,
        input_complete_nonterminal_quads_len,
        output,
        output_complete_quad_len,
        decode_table,
        decode_allow_trailing_bits,
        padding_mode,
    )
}

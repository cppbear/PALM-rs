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
pub struct GeneralPurposeEstimate {
    /// input len % 4
    rem: usize,
    conservative_decoded_len: usize,
}
#[derive(Clone, Copy, Debug)]
pub struct GeneralPurposeConfig {
    encode_padding: bool,
    decode_allow_trailing_bits: bool,
    decode_padding_mode: DecodePaddingMode,
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
    ) -> Result<DecodeMetadata, DecodeSliceError> {}
    fn config(&self) -> &Self::Config {
        &self.config
    }
}
impl GeneralPurposeEstimate {
    pub(crate) fn new(encoded_len: usize) -> Self {
        let rem = encoded_len % 4;
        Self {
            rem,
            conservative_decoded_len: (encoded_len / 4 + usize::from(rem > 0)) * 3,
        }
    }
}

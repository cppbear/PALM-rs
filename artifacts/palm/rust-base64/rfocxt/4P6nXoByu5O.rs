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
    fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
        let mut input_index: usize = 0;
        const BLOCKS_PER_FAST_LOOP: usize = 4;
        const LOW_SIX_BITS: u64 = 0x3F;
        let last_fast_index = input.len().saturating_sub(BLOCKS_PER_FAST_LOOP * 6 + 2);
        let mut output_index = 0;
        if last_fast_index > 0 {
            while input_index <= last_fast_index {
                let input_chunk = &input[input_index..(input_index
                    + (BLOCKS_PER_FAST_LOOP * 6 + 2))];
                let output_chunk = &mut output[output_index..(output_index
                    + BLOCKS_PER_FAST_LOOP * 8)];
                let input_u64 = read_u64(&input_chunk[0..]);
                output_chunk[0] = self
                    .encode_table[((input_u64 >> 58) & LOW_SIX_BITS) as usize];
                output_chunk[1] = self
                    .encode_table[((input_u64 >> 52) & LOW_SIX_BITS) as usize];
                output_chunk[2] = self
                    .encode_table[((input_u64 >> 46) & LOW_SIX_BITS) as usize];
                output_chunk[3] = self
                    .encode_table[((input_u64 >> 40) & LOW_SIX_BITS) as usize];
                output_chunk[4] = self
                    .encode_table[((input_u64 >> 34) & LOW_SIX_BITS) as usize];
                output_chunk[5] = self
                    .encode_table[((input_u64 >> 28) & LOW_SIX_BITS) as usize];
                output_chunk[6] = self
                    .encode_table[((input_u64 >> 22) & LOW_SIX_BITS) as usize];
                output_chunk[7] = self
                    .encode_table[((input_u64 >> 16) & LOW_SIX_BITS) as usize];
                let input_u64 = read_u64(&input_chunk[6..]);
                output_chunk[8] = self
                    .encode_table[((input_u64 >> 58) & LOW_SIX_BITS) as usize];
                output_chunk[9] = self
                    .encode_table[((input_u64 >> 52) & LOW_SIX_BITS) as usize];
                output_chunk[10] = self
                    .encode_table[((input_u64 >> 46) & LOW_SIX_BITS) as usize];
                output_chunk[11] = self
                    .encode_table[((input_u64 >> 40) & LOW_SIX_BITS) as usize];
                output_chunk[12] = self
                    .encode_table[((input_u64 >> 34) & LOW_SIX_BITS) as usize];
                output_chunk[13] = self
                    .encode_table[((input_u64 >> 28) & LOW_SIX_BITS) as usize];
                output_chunk[14] = self
                    .encode_table[((input_u64 >> 22) & LOW_SIX_BITS) as usize];
                output_chunk[15] = self
                    .encode_table[((input_u64 >> 16) & LOW_SIX_BITS) as usize];
                let input_u64 = read_u64(&input_chunk[12..]);
                output_chunk[16] = self
                    .encode_table[((input_u64 >> 58) & LOW_SIX_BITS) as usize];
                output_chunk[17] = self
                    .encode_table[((input_u64 >> 52) & LOW_SIX_BITS) as usize];
                output_chunk[18] = self
                    .encode_table[((input_u64 >> 46) & LOW_SIX_BITS) as usize];
                output_chunk[19] = self
                    .encode_table[((input_u64 >> 40) & LOW_SIX_BITS) as usize];
                output_chunk[20] = self
                    .encode_table[((input_u64 >> 34) & LOW_SIX_BITS) as usize];
                output_chunk[21] = self
                    .encode_table[((input_u64 >> 28) & LOW_SIX_BITS) as usize];
                output_chunk[22] = self
                    .encode_table[((input_u64 >> 22) & LOW_SIX_BITS) as usize];
                output_chunk[23] = self
                    .encode_table[((input_u64 >> 16) & LOW_SIX_BITS) as usize];
                let input_u64 = read_u64(&input_chunk[18..]);
                output_chunk[24] = self
                    .encode_table[((input_u64 >> 58) & LOW_SIX_BITS) as usize];
                output_chunk[25] = self
                    .encode_table[((input_u64 >> 52) & LOW_SIX_BITS) as usize];
                output_chunk[26] = self
                    .encode_table[((input_u64 >> 46) & LOW_SIX_BITS) as usize];
                output_chunk[27] = self
                    .encode_table[((input_u64 >> 40) & LOW_SIX_BITS) as usize];
                output_chunk[28] = self
                    .encode_table[((input_u64 >> 34) & LOW_SIX_BITS) as usize];
                output_chunk[29] = self
                    .encode_table[((input_u64 >> 28) & LOW_SIX_BITS) as usize];
                output_chunk[30] = self
                    .encode_table[((input_u64 >> 22) & LOW_SIX_BITS) as usize];
                output_chunk[31] = self
                    .encode_table[((input_u64 >> 16) & LOW_SIX_BITS) as usize];
                output_index += BLOCKS_PER_FAST_LOOP * 8;
                input_index += BLOCKS_PER_FAST_LOOP * 6;
            }
        }
        const LOW_SIX_BITS_U8: u8 = 0x3F;
        let rem = input.len() % 3;
        let start_of_rem = input.len() - rem;
        while input_index < start_of_rem {
            let input_chunk = &input[input_index..(input_index + 3)];
            let output_chunk = &mut output[output_index..(output_index + 4)];
            output_chunk[0] = self.encode_table[(input_chunk[0] >> 2) as usize];
            output_chunk[1] = self
                .encode_table[((input_chunk[0] << 4 | input_chunk[1] >> 4)
                & LOW_SIX_BITS_U8) as usize];
            output_chunk[2] = self
                .encode_table[((input_chunk[1] << 2 | input_chunk[2] >> 6)
                & LOW_SIX_BITS_U8) as usize];
            output_chunk[3] = self
                .encode_table[(input_chunk[2] & LOW_SIX_BITS_U8) as usize];
            input_index += 3;
            output_index += 4;
        }
        if rem == 2 {
            output[output_index] = self
                .encode_table[(input[start_of_rem] >> 2) as usize];
            output[output_index + 1] = self
                .encode_table[((input[start_of_rem] << 4 | input[start_of_rem + 1] >> 4)
                & LOW_SIX_BITS_U8) as usize];
            output[output_index + 2] = self
                .encode_table[((input[start_of_rem + 1] << 2) & LOW_SIX_BITS_U8)
                as usize];
            output_index += 3;
        } else if rem == 1 {
            output[output_index] = self
                .encode_table[(input[start_of_rem] >> 2) as usize];
            output[output_index + 1] = self
                .encode_table[((input[start_of_rem] << 4) & LOW_SIX_BITS_U8) as usize];
            output_index += 2;
        }
        output_index
    }
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
#[inline]
fn read_u64(s: &[u8]) -> u64 {
    u64::from_be_bytes(s[..8].try_into().unwrap())
}

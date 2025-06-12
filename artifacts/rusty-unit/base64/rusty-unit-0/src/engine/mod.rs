//! Provides the [Engine] abstraction and out of the box implementations.
#[cfg(any(feature = "alloc", test))]
use crate::chunked_encoder;
use crate::{
    encode::{encode_with_padding, EncodeSliceError},
    encoded_len, DecodeError, DecodeSliceError,
};
#[cfg(any(feature = "alloc", test))]
use alloc::vec::Vec;

#[cfg(any(feature = "alloc", test))]
use alloc::{string::String, vec};

pub mod general_purpose;

#[cfg(test)]
mod naive;

#[cfg(test)]
mod tests;

pub use general_purpose::{GeneralPurpose, GeneralPurposeConfig};

/// An `Engine` provides low-level encoding and decoding operations that all other higher-level parts of the API use. Users of the library will generally not need to implement this.
///
/// Different implementations offer different characteristics. The library currently ships with
/// [GeneralPurpose] that offers good speed and works on any CPU, with more choices
/// coming later, like a constant-time one when side channel resistance is called for, and vendor-specific vectorized ones for more speed.
///
/// See [general_purpose::STANDARD_NO_PAD] if you just want standard base64. Otherwise, when possible, it's
/// recommended to store the engine in a `const` so that references to it won't pose any lifetime
/// issues, and to avoid repeating the cost of engine setup.
///
/// Since almost nobody will need to implement `Engine`, docs for internal methods are hidden.
// When adding an implementation of Engine, include them in the engine test suite:
// - add an implementation of [engine::tests::EngineWrapper]
// - add the implementation to the `all_engines` macro
// All tests run on all engines listed in the macro.
pub trait Engine: Send + Sync {
    /// The config type used by this engine
    type Config: Config;
    /// The decode estimate used by this engine
    type DecodeEstimate: DecodeEstimate;

    /// This is not meant to be called directly; it is only for `Engine` implementors.
    /// See the other `encode*` functions on this trait.
    ///
    /// Encode the `input` bytes into the `output` buffer based on the mapping in `encode_table`.
    ///
    /// `output` will be long enough to hold the encoded data.
    ///
    /// Returns the number of bytes written.
    ///
    /// No padding should be written; that is handled separately.
    ///
    /// Must not write any bytes into the output slice other than the encoded data.
    #[doc(hidden)]
    fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize;

    /// This is not meant to be called directly; it is only for `Engine` implementors.
    ///
    /// As an optimization to prevent the decoded length from being calculated twice, it is
    /// sometimes helpful to have a conservative estimate of the decoded size before doing the
    /// decoding, so this calculation is done separately and passed to [Engine::decode()] as needed.
    #[doc(hidden)]
    fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate;

    /// This is not meant to be called directly; it is only for `Engine` implementors.
    /// See the other `decode*` functions on this trait.
    ///
    /// Decode `input` base64 bytes into the `output` buffer.
    ///
    /// `decode_estimate` is the result of [Engine::internal_decoded_len_estimate()], which is passed in to avoid
    /// calculating it again (expensive on short inputs).`
    ///
    /// Each complete 4-byte chunk of encoded data decodes to 3 bytes of decoded data, but this
    /// function must also handle the final possibly partial chunk.
    /// If the input length is not a multiple of 4, or uses padding bytes to reach a multiple of 4,
    /// the trailing 2 or 3 bytes must decode to 1 or 2 bytes, respectively, as per the
    /// [RFC](https://tools.ietf.org/html/rfc4648#section-3.5).
    ///
    /// Decoding must not write any bytes into the output slice other than the decoded data.
    ///
    /// Non-canonical trailing bits in the final tokens or non-canonical padding must be reported as
    /// errors unless the engine is configured otherwise.
    #[doc(hidden)]
    fn internal_decode(
        &self,
        input: &[u8],
        output: &mut [u8],
        decode_estimate: Self::DecodeEstimate,
    ) -> Result<DecodeMetadata, DecodeSliceError>;

    /// Returns the config for this engine.
    fn config(&self) -> &Self::Config;

    /// Encode arbitrary octets as base64 using the provided `Engine`.
    /// Returns a `String`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use base64::{Engine as _, engine::{self, general_purpose}, alphabet};
    ///
    /// let b64 = general_purpose::STANDARD.encode(b"hello world~");
    /// println!("{}", b64);
    ///
    /// const CUSTOM_ENGINE: engine::GeneralPurpose =
    ///     engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
    ///
    /// let b64_url = CUSTOM_ENGINE.encode(b"hello internet~");
    /// ```
    #[cfg(any(feature = "alloc", test))]
    #[inline]
    fn encode<T: AsRef<[u8]>>(&self, input: T) -> String {
        fn inner<E>(engine: &E, input_bytes: &[u8]) -> String
        where
            E: Engine + ?Sized,
        {
            let encoded_size = encoded_len(input_bytes.len(), engine.config().encode_padding())
                .expect("integer overflow when calculating buffer size");

            let mut buf = vec![0; encoded_size];

            encode_with_padding(input_bytes, &mut buf[..], engine, encoded_size);

            String::from_utf8(buf).expect("Invalid UTF8")
        }

        inner(self, input.as_ref())
    }

    /// Encode arbitrary octets as base64 into a supplied `String`.
    /// Writes into the supplied `String`, which may allocate if its internal buffer isn't big enough.
    ///
    /// # Example
    ///
    /// ```rust
    /// use base64::{Engine as _, engine::{self, general_purpose}, alphabet};
    /// const CUSTOM_ENGINE: engine::GeneralPurpose =
    ///     engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
    ///
    /// fn main() {
    ///     let mut buf = String::new();
    ///     general_purpose::STANDARD.encode_string(b"hello world~", &mut buf);
    ///     println!("{}", buf);
    ///
    ///     buf.clear();
    ///     CUSTOM_ENGINE.encode_string(b"hello internet~", &mut buf);
    ///     println!("{}", buf);
    /// }
    /// ```
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

        inner(self, input.as_ref(), output_buf)
    }

    /// Encode arbitrary octets as base64 into a supplied slice.
    /// Writes into the supplied output buffer.
    ///
    /// This is useful if you wish to avoid allocation entirely (e.g. encoding into a stack-resident
    /// or statically-allocated buffer).
    ///
    /// # Example
    ///
    #[cfg_attr(feature = "alloc", doc = "```")]
    #[cfg_attr(not(feature = "alloc"), doc = "```ignore")]
    /// use base64::{Engine as _, engine::general_purpose};
    /// let s = b"hello internet!";
    /// let mut buf = Vec::new();
    /// // make sure we'll have a slice big enough for base64 + padding
    /// buf.resize(s.len() * 4 / 3 + 4, 0);
    ///
    /// let bytes_written = general_purpose::STANDARD.encode_slice(s, &mut buf).unwrap();
    ///
    /// // shorten our vec down to just what was written
    /// buf.truncate(bytes_written);
    ///
    /// assert_eq!(s, general_purpose::STANDARD.decode(&buf).unwrap().as_slice());
    /// ```
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
            let encoded_size = encoded_len(input_bytes.len(), engine.config().encode_padding())
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

    /// Decode the input into a new `Vec`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use base64::{Engine as _, alphabet, engine::{self, general_purpose}};
    ///
    /// let bytes = general_purpose::STANDARD
    ///     .decode("aGVsbG8gd29ybGR+Cg==").unwrap();
    /// println!("{:?}", bytes);
    ///
    /// // custom engine setup
    /// let bytes_url = engine::GeneralPurpose::new(
    ///              &alphabet::URL_SAFE,
    ///              general_purpose::NO_PAD)
    ///     .decode("aGVsbG8gaW50ZXJuZXR-Cg").unwrap();
    /// println!("{:?}", bytes_url);
    /// ```
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

    /// Decode the `input` into the supplied `buffer`.
    ///
    /// Writes into the supplied `Vec`, which may allocate if its internal buffer isn't big enough.
    /// Returns a `Result` containing an empty tuple, aka `()`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use base64::{Engine as _, alphabet, engine::{self, general_purpose}};
    /// const CUSTOM_ENGINE: engine::GeneralPurpose =
    ///     engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::PAD);
    ///
    /// fn main() {
    ///     use base64::Engine;
    ///     let mut buffer = Vec::<u8>::new();
    ///     // with the default engine
    ///     general_purpose::STANDARD
    ///         .decode_vec("aGVsbG8gd29ybGR+Cg==", &mut buffer,).unwrap();
    ///     println!("{:?}", buffer);
    ///
    ///     buffer.clear();
    ///
    ///     // with a custom engine
    ///     CUSTOM_ENGINE.decode_vec(
    ///         "aGVsbG8gaW50ZXJuZXR-Cg==",
    ///         &mut buffer,
    ///     ).unwrap();
    ///     println!("{:?}", buffer);
    /// }
    /// ```
    #[cfg(any(feature = "alloc", test))]
    #[inline]
    fn decode_vec<T: AsRef<[u8]>>(
        &self,
        input: T,
        buffer: &mut Vec<u8>,
    ) -> Result<(), DecodeError> {
        fn inner<E>(engine: &E, input_bytes: &[u8], buffer: &mut Vec<u8>) -> Result<(), DecodeError>
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

    /// Decode the input into the provided output slice.
    ///
    /// Returns the number of bytes written to the slice, or an error if `output` is smaller than
    /// the estimated decoded length.
    ///
    /// This will not write any bytes past exactly what is decoded (no stray garbage bytes at the end).
    ///
    /// See [crate::decoded_len_estimate] for calculating buffer sizes.
    ///
    /// See [Engine::decode_slice_unchecked] for a version that panics instead of returning an error
    /// if the output buffer is too small.
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

    /// Decode the input into the provided output slice.
    ///
    /// Returns the number of bytes written to the slice.
    ///
    /// This will not write any bytes past exactly what is decoded (no stray garbage bytes at the end).
    ///
    /// See [crate::decoded_len_estimate] for calculating buffer sizes.
    ///
    /// See [Engine::decode_slice] for a version that returns an error instead of panicking if the output
    /// buffer is too small.
    ///
    /// # Panics
    ///
    /// Panics if the provided output buffer is too small for the decoded data.
    #[inline]
    fn decode_slice_unchecked<T: AsRef<[u8]>>(
        &self,
        input: T,
        output: &mut [u8],
    ) -> Result<usize, DecodeError> {
        fn inner<E>(engine: &E, input_bytes: &[u8], output: &mut [u8]) -> Result<usize, DecodeError>
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

/// The minimal level of configuration that engines must support.
pub trait Config {
    /// Returns `true` if padding should be added after the encoded output.
    ///
    /// Padding is added outside the engine's encode() since the engine may be used
    /// to encode only a chunk of the overall output, so it can't always know when
    /// the output is "done" and would therefore need padding (if configured).
    // It could be provided as a separate parameter when encoding, but that feels like
    // leaking an implementation detail to the user, and it's hopefully more convenient
    // to have to only pass one thing (the engine) to any part of the API.
    fn encode_padding(&self) -> bool;
}

/// The decode estimate used by an engine implementation. Users do not need to interact with this;
/// it is only for engine implementors.
///
/// Implementors may store relevant data here when constructing this to avoid having to calculate
/// them again during actual decoding.
pub trait DecodeEstimate {
    /// Returns a conservative (err on the side of too big) estimate of the decoded length to use
    /// for pre-allocating buffers, etc.
    ///
    /// The estimate must be no larger than the next largest complete triple of decoded bytes.
    /// That is, the final quad of tokens to decode may be assumed to be complete with no padding.
    fn decoded_len_estimate(&self) -> usize;
}

/// Controls how pad bytes are handled when decoding.
///
/// Each [Engine] must support at least the behavior indicated by
/// [DecodePaddingMode::RequireCanonical], and may support other modes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecodePaddingMode {
    /// Canonical padding is allowed, but any fewer padding bytes than that is also allowed.
    Indifferent,
    /// Padding must be canonical (0, 1, or 2 `=` as needed to produce a 4 byte suffix).
    RequireCanonical,
    /// Padding must be absent -- for when you want predictable padding, without any wasted bytes.
    RequireNone,
}

/// Metadata about the result of a decode operation
#[derive(PartialEq, Eq, Debug)]
pub struct DecodeMetadata {
    /// Number of decoded bytes output
    pub(crate) decoded_len: usize,
    /// Offset of the first padding byte in the input, if any
    pub(crate) padding_offset: Option<usize>,
}

impl DecodeMetadata {
    pub(crate) fn new(decoded_bytes: usize, padding_index: Option<usize>) -> Self {
        Self {
            decoded_len: decoded_bytes,
            padding_offset: padding_index,
        }
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use engine::Config;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::error::Error;
	use std::convert::TryFrom;
	use std::cmp::Eq;
	use std::convert::From;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1() {
    rusty_monitor::set_test_id(1);
    let mut usize_0: usize = 53usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut usize_1: usize = 4007usize;
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata::new(usize_1, option_0);
    let mut decodemetadata_0_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_0;
    let mut bool_0: bool = true;
    let mut usize_2: usize = 4519usize;
    let mut option_1: std::option::Option<usize> = crate::encode::encoded_len(usize_2, bool_0);
    let mut usize_3: usize = 5026usize;
    let mut decodemetadata_1: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata::new(usize_3, option_1);
    let mut decodemetadata_1_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_1;
    let mut bool_1: bool = true;
    let mut usize_4: usize = 934usize;
    let mut option_2: std::option::Option<usize> = crate::encode::encoded_len(usize_4, bool_1);
    let mut usize_5: usize = 9490usize;
    let mut decodemetadata_2: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata {decoded_len: usize_5, padding_offset: option_2};
    let mut decodemetadata_2_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_2;
    let mut usize_6: usize = 38usize;
    let mut option_3: std::option::Option<usize> = std::option::Option::Some(usize_6);
    let mut usize_7: usize = 1364usize;
    let mut decodemetadata_3: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata {decoded_len: usize_7, padding_offset: option_3};
    let mut decodemetadata_3_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_3;
    let mut u8_0: u8 = 58u8;
    let mut usize_8: usize = 6238usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_8, u8_0);
    let mut usize_9: usize = 6349usize;
    let mut option_4: std::option::Option<usize> = std::option::Option::Some(usize_9);
    let mut usize_10: usize = 8484usize;
    let mut decodemetadata_4: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata {decoded_len: usize_10, padding_offset: option_4};
    let mut decodemetadata_4_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_4;
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_0);
    let mut tuple_0: () = crate::engine::DecodeMetadata::assert_receiver_is_total_eq(decodemetadata_3_ref_0);
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut tuple_1: () = crate::decode::DecodeSliceError::assert_receiver_is_total_eq(decodesliceerror_0_ref_0);
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    let mut tuple_2: () = crate::engine::DecodeMetadata::assert_receiver_is_total_eq(decodemetadata_2_ref_0);
    let mut bool_2: bool = crate::engine::DecodeMetadata::eq(decodemetadata_1_ref_0, decodemetadata_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2() {
    rusty_monitor::set_test_id(2);
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut u8_0: u8 = 101u8;
    let mut usize_0: usize = 5606usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_0, u8_0);
    let mut decodeerror_1_ref_0: &decode::DecodeError = &mut decodeerror_1;
    let mut str_0: &str = "1DvKRiuXzCuHed4f";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireNone;
    let mut decodepaddingmode_0_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_0;
    let mut u8_1: u8 = 51u8;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::ReservedByte(u8_1);
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut usize_1: usize = 1229usize;
    let mut usize_2: usize = 9753usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_2);
    let mut usize_3: usize = 8313usize;
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata::new(usize_3, option_0);
    let mut decodemetadata_0_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_0;
    let mut bool_0: bool = true;
    let mut usize_4: usize = 9400usize;
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::OutputSliceTooSmall;
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut usize_5: usize = 9488usize;
    let mut option_1: std::option::Option<usize> = crate::encode::encoded_len(usize_4, bool_0);
    let mut tuple_0: () = crate::engine::DecodeMetadata::assert_receiver_is_total_eq(decodemetadata_0_ref_0);
    let mut usize_6: usize = std::option::Option::unwrap(option_1);
    let mut decodepaddingmode_1: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::clone(decodepaddingmode_0_ref_0);
    let mut result_0: std::result::Result<crate::alphabet::Alphabet, alphabet::ParseAlphabetError> = crate::alphabet::Alphabet::try_from(str_0_ref_0);
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_0);
    let mut alphabet_0: crate::alphabet::Alphabet = std::result::Result::unwrap(result_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4() {
    rusty_monitor::set_test_id(4);
    let mut u8_0: u8 = 106u8;
    let mut usize_0: usize = 2399usize;
    let mut str_0: &str = "AoxRDm2qORWuptJ";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireCanonical;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_padding_mode(generalpurposeconfig_0, decodepaddingmode_0);
    let mut generalpurposeconfig_1_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_1;
    let mut u8_1: u8 = 23u8;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::DuplicatedByte(u8_1);
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut bool_0: bool = false;
    let mut generalpurposeconfig_2: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    let mut generalpurposeconfig_3: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_encode_padding(generalpurposeconfig_2, bool_0);
    let mut generalpurposeconfig_3_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_3;
    let mut u8_2: u8 = 36u8;
    let mut usize_1: usize = 6635usize;
    let mut u8_3: u8 = 54u8;
    let mut usize_2: usize = 7772usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_2, u8_3);
    let mut decodeerror_0_ref_0: &decode::DecodeError = &mut decodeerror_0;
    let mut parsealphabeterror_1: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::DuplicatedByte(u8_2);
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut generalpurposeconfig_4: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::clone(generalpurposeconfig_3_ref_0);
    let mut bool_1: bool = crate::engine::general_purpose::GeneralPurposeConfig::encode_padding(generalpurposeconfig_1_ref_0);
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodepaddingmode_1: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireCanonical;
    let mut generalpurposeconfig_4_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_4;
    let mut generalpurposeconfig_5: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::clone(generalpurposeconfig_4_ref_0);
    let mut result_0: std::result::Result<crate::alphabet::Alphabet, alphabet::ParseAlphabetError> = crate::alphabet::Alphabet::try_from(str_0_ref_0);
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_0, u8_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5() {
    rusty_monitor::set_test_id(5);
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_0);
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_1);
    let mut decodesliceerror_1_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_1;
    let mut u8_0: u8 = 114u8;
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut bool_0: bool = true;
    let mut usize_0: usize = 3786usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut usize_1: usize = 8041usize;
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata {decoded_len: usize_1, padding_offset: option_0};
    let mut decodemetadata_0_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_0;
    let mut usize_2: usize = 837usize;
    let mut option_1: std::option::Option<usize> = std::option::Option::Some(usize_2);
    let mut usize_3: usize = 8092usize;
    let mut decodemetadata_1: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata {decoded_len: usize_3, padding_offset: option_1};
    let mut decodemetadata_1_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_1;
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut decodepaddingmode_0_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_0;
    let mut str_0: &str = "WuHa";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut result_0: std::result::Result<crate::alphabet::Alphabet, alphabet::ParseAlphabetError> = crate::alphabet::Alphabet::try_from(str_0_ref_0);
    let mut decodepaddingmode_1: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::clone(decodepaddingmode_0_ref_0);
    let mut bool_1: bool = crate::engine::DecodeMetadata::eq(decodemetadata_1_ref_0, decodemetadata_0_ref_0);
    let mut option_2: std::option::Option<usize> = crate::encode::encoded_len(usize_0, bool_0);
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::UnprintableByte(u8_0);
    let mut alphabet_0: crate::alphabet::Alphabet = std::result::Result::unwrap(result_0);
    let mut decodepaddingmode_1_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_1;
    let mut tuple_0: () = crate::engine::DecodePaddingMode::assert_receiver_is_total_eq(decodepaddingmode_1_ref_0);
    let mut bool_2: bool = crate::decode::DecodeSliceError::eq(decodesliceerror_1_ref_0, decodesliceerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6() {
    rusty_monitor::set_test_id(6);
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut usize_0: usize = 5657usize;
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata::new(usize_0, option_0);
    let mut decodemetadata_0_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_0;
    let mut bool_0: bool = false;
    let mut usize_1: usize = 8513usize;
    let mut option_1: std::option::Option<usize> = crate::encode::encoded_len(usize_1, bool_0);
    let mut usize_2: usize = 3106usize;
    let mut u8_0: u8 = 63u8;
    let mut usize_3: usize = 2635usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_3, u8_0);
    let mut decodeerror_0_ref_0: &decode::DecodeError = &mut decodeerror_0;
    let mut u8_1: u8 = 117u8;
    let mut usize_4: usize = 2396usize;
    let mut u8_2: u8 = 53u8;
    let mut usize_5: usize = 7488usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_5, u8_2);
    let mut decodeerror_1_ref_0: &decode::DecodeError = &mut decodeerror_1;
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodeerror_2_ref_0: &decode::DecodeError = &mut decodeerror_2;
    let mut u8_3: u8 = 96u8;
    let mut usize_6: usize = 1031usize;
    let mut decodeerror_3: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_6, u8_3);
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_3);
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut option_2: std::option::Option<&dyn std::error::Error> = crate::decode::DecodeSliceError::source(decodesliceerror_0_ref_0);
    let mut tuple_0: () = crate::decode::DecodeError::assert_receiver_is_total_eq(decodeerror_2_ref_0);
    let mut tuple_1: () = crate::decode::DecodeError::assert_receiver_is_total_eq(decodeerror_1_ref_0);
    let mut decodeerror_4: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_4, u8_1);
    let mut decodeerror_5: decode::DecodeError = crate::decode::DecodeError::clone(decodeerror_0_ref_0);
    let mut decodemetadata_1: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata {decoded_len: usize_2, padding_offset: option_1};
    let mut decodeerror_6: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodemetadata_1_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_1;
    let mut bool_1: bool = crate::engine::DecodeMetadata::eq(decodemetadata_1_ref_0, decodemetadata_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8() {
    rusty_monitor::set_test_id(8);
    let mut u8_0: u8 = 8u8;
    let mut usize_0: usize = 6505usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_0, u8_0);
    let mut decodeerror_0_ref_0: &decode::DecodeError = &mut decodeerror_0;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodeerror_1_ref_0: &decode::DecodeError = &mut decodeerror_1;
    let mut bool_0: bool = false;
    let mut usize_1: usize = 1685usize;
    let mut option_0: std::option::Option<usize> = crate::encode::encoded_len(usize_1, bool_0);
    let mut usize_2: usize = 6686usize;
    let mut bool_1: bool = false;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireCanonical;
    let mut decodepaddingmode_0_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_0;
    let mut u8_1: u8 = 72u8;
    let mut usize_3: usize = 7767usize;
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_3, u8_1);
    let mut decodeerror_2_ref_0: &decode::DecodeError = &mut decodeerror_2;
    let mut decodeerror_3: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_3);
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut decodepaddingmode_1: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::clone(decodesliceerror_0_ref_0);
    let mut decodeerror_4: decode::DecodeError = crate::decode::DecodeError::clone(decodeerror_2_ref_0);
    let mut decodeerror_4_ref_0: &decode::DecodeError = &mut decodeerror_4;
    let mut tuple_0: () = crate::decode::DecodeError::assert_receiver_is_total_eq(decodeerror_4_ref_0);
    let mut decodepaddingmode_1_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_1;
    let mut tuple_1: () = crate::engine::DecodePaddingMode::assert_receiver_is_total_eq(decodepaddingmode_1_ref_0);
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_encode_padding(generalpurposeconfig_0, bool_1);
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata::new(usize_2, option_0);
    let mut decodesliceerror_1_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_1;
    let mut tuple_2: () = crate::decode::DecodeSliceError::assert_receiver_is_total_eq(decodesliceerror_1_ref_0);
    let mut bool_2: bool = crate::decode::DecodeError::eq(decodeerror_1_ref_0, decodeerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_12() {
    rusty_monitor::set_test_id(12);
    let mut usize_0: usize = 3068usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_0);
    let mut decodeerror_0_ref_0: &decode::DecodeError = &mut decodeerror_0;
    let mut bool_0: bool = false;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::InvalidLength;
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut usize_1: usize = 8208usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_1);
    let mut usize_2: usize = 2203usize;
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata {decoded_len: usize_2, padding_offset: option_0};
    let mut decodemetadata_0_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_0;
    let mut usize_3: usize = 9872usize;
    let mut usize_4: usize = 8441usize;
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut usize_5: usize = 1439usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_5);
    let mut decodeerror_1_ref_0: &decode::DecodeError = &mut decodeerror_1;
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::OutputSliceTooSmall;
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut u8_0: u8 = 106u8;
    let mut usize_6: usize = 9507usize;
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_6, u8_0);
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_2);
    let mut decodesliceerror_1_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_1;
    let mut decodeerror_3: decode::DecodeError = crate::decode::DecodeError::clone(decodeerror_1_ref_0);
    let mut decodeerror_4: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_4);
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::new();
    let mut tuple_0: () = crate::engine::DecodeMetadata::assert_receiver_is_total_eq(decodemetadata_0_ref_0);
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_allow_trailing_bits(generalpurposeconfig_0, bool_0);
    let mut parsealphabeterror_1: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::InvalidLength;
    let mut tuple_1: () = crate::decode::DecodeError::assert_receiver_is_total_eq(decodeerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_13() {
    rusty_monitor::set_test_id(13);
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::OutputSliceTooSmall;
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut decodepaddingmode_1: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireCanonical;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::new();
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_padding_mode(generalpurposeconfig_0, decodepaddingmode_1);
    let mut generalpurposeconfig_1_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_1;
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut usize_0: usize = 3591usize;
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata::new(usize_0, option_0);
    let mut decodemetadata_0_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_0;
    let mut usize_1: usize = 5146usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_1);
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_0);
    let mut decodesliceerror_1_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_1;
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut decodepaddingmode_2: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut decodepaddingmode_2_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_2;
    let mut decodepaddingmode_3: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::clone(decodepaddingmode_2_ref_0);
    let mut decodepaddingmode_3_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_3;
    let mut tuple_0: () = crate::engine::DecodePaddingMode::assert_receiver_is_total_eq(decodepaddingmode_3_ref_0);
    let mut tuple_1: () = crate::engine::DecodeMetadata::assert_receiver_is_total_eq(decodemetadata_0_ref_0);
    let mut generalpurposeconfig_2: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::clone(generalpurposeconfig_1_ref_0);
    let mut generalpurposeconfig_3: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_padding_mode(generalpurposeconfig_2, decodepaddingmode_0);
    let mut generalpurposeconfig_4: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut generalpurposeconfig_3_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_3;
    let mut generalpurposeconfig_5: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::clone(generalpurposeconfig_3_ref_0);
    let mut generalpurposeconfig_5_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_5;
    let mut generalpurposeconfig_6: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::clone(generalpurposeconfig_5_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_15() {
    rusty_monitor::set_test_id(15);
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut decodepaddingmode_0_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_0;
    let mut bool_0: bool = true;
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::OutputSliceTooSmall;
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut usize_0: usize = 7615usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut usize_1: usize = 7210usize;
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata {decoded_len: usize_1, padding_offset: option_0};
    let mut decodemetadata_0_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_0;
    let mut bool_1: bool = false;
    let mut usize_2: usize = 7174usize;
    let mut option_1: std::option::Option<usize> = crate::encode::encoded_len(usize_2, bool_1);
    let mut usize_3: usize = 9760usize;
    let mut decodemetadata_1: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata::new(usize_3, option_1);
    let mut decodemetadata_1_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_1;
    let mut u8_0: u8 = 112u8;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::DuplicatedByte(u8_0);
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::new();
    let mut tuple_0: () = crate::engine::DecodeMetadata::assert_receiver_is_total_eq(decodemetadata_0_ref_0);
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::new();
    let mut generalpurposeconfig_2: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::new();
    let mut decodepaddingmode_1: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireNone;
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::clone(decodesliceerror_0_ref_0);
    let mut decodesliceerror_1_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_1;
    let mut decodesliceerror_2: decode::DecodeSliceError = crate::decode::DecodeSliceError::clone(decodesliceerror_1_ref_0);
    let mut generalpurposeconfig_3: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_allow_trailing_bits(generalpurposeconfig_2, bool_0);
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut tuple_1: () = crate::engine::DecodePaddingMode::assert_receiver_is_total_eq(decodepaddingmode_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_19() {
    rusty_monitor::set_test_id(19);
    let mut bool_0: bool = false;
    let mut usize_0: usize = 5532usize;
    let mut option_0: std::option::Option<usize> = crate::encode::encoded_len(usize_0, bool_0);
    let mut usize_1: usize = 7009usize;
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata {decoded_len: usize_1, padding_offset: option_0};
    let mut decodemetadata_0_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_0;
    let mut bool_1: bool = true;
    let mut usize_2: usize = 6366usize;
    let mut option_1: std::option::Option<usize> = crate::encode::encoded_len(usize_2, bool_1);
    let mut usize_3: usize = 5806usize;
    let mut decodemetadata_1: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata::new(usize_3, option_1);
    let mut decodemetadata_1_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_1;
    let mut u8_0: u8 = 43u8;
    let mut usize_4: usize = 7088usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_4, u8_0);
    let mut decodeerror_0_ref_0: &decode::DecodeError = &mut decodeerror_0;
    let mut u8_1: u8 = 113u8;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::ReservedByte(u8_1);
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut u8_2: u8 = 109u8;
    let mut usize_5: usize = 5671usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_5, u8_2);
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_1);
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::OutputSliceTooSmall;
    let mut decodesliceerror_1_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_1;
    let mut option_2: std::option::Option<&dyn std::error::Error> = crate::decode::DecodeSliceError::source(decodesliceerror_0_ref_0);
    let mut error_0: &dyn std::error::Error = std::option::Option::unwrap(option_2);
    let mut bool_2: bool = crate::engine::DecodeMetadata::eq(decodemetadata_1_ref_0, decodemetadata_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_22() {
    rusty_monitor::set_test_id(22);
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut decodepaddingmode_0_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_0;
    let mut decodepaddingmode_1: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut decodepaddingmode_1_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_1;
    let mut decodepaddingmode_2: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireNone;
    let mut decodepaddingmode_2_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_2;
    let mut u8_0: u8 = 45u8;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::ReservedByte(u8_0);
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodeerror_0_ref_0: &decode::DecodeError = &mut decodeerror_0;
    let mut usize_0: usize = 3294usize;
    let mut decodepaddingmode_3: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireNone;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    let mut u8_1: u8 = 95u8;
    let mut usize_1: usize = 3377usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_1, u8_1);
    let mut decodeerror_1_ref_0: &decode::DecodeError = &mut decodeerror_1;
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut usize_2: usize = 589usize;
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata::new(usize_2, option_0);
    let mut decodemetadata_0_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_0;
    let mut decodepaddingmode_4: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut decodepaddingmode_4_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_4;
    let mut tuple_0: () = crate::engine::DecodePaddingMode::assert_receiver_is_total_eq(decodepaddingmode_4_ref_0);
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::clone(decodeerror_1_ref_0);
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_padding_mode(generalpurposeconfig_0, decodepaddingmode_3);
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::OutputSliceTooSmall;
    let mut decodeerror_3: decode::DecodeError = crate::decode::DecodeError::clone(decodeerror_0_ref_0);
    let mut generalpurposeconfig_1_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_1;
    let mut tuple_1: () = crate::engine::DecodePaddingMode::assert_receiver_is_total_eq(decodepaddingmode_2_ref_0);
    let mut bool_0: bool = crate::engine::DecodePaddingMode::eq(decodepaddingmode_1_ref_0, decodepaddingmode_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_23() {
    rusty_monitor::set_test_id(23);
    let mut u8_0: u8 = 112u8;
    let mut usize_0: usize = 6869usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_0, u8_0);
    let mut decodeerror_0_ref_0: &decode::DecodeError = &mut decodeerror_0;
    let mut usize_1: usize = 3717usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_1);
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_1);
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::InvalidLength;
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut u8_1: u8 = 32u8;
    let mut parsealphabeterror_1: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::ReservedByte(u8_1);
    let mut parsealphabeterror_1_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_1;
    let mut bool_0: bool = false;
    let mut usize_2: usize = 212usize;
    let mut option_0: std::option::Option<usize> = crate::encode::encoded_len(usize_2, bool_0);
    let mut usize_3: usize = 5182usize;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::new();
    let mut generalpurposeconfig_0_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_0;
    let mut bool_1: bool = false;
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    let mut generalpurposeconfig_2: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_allow_trailing_bits(generalpurposeconfig_1, bool_1);
    let mut generalpurposeconfig_2_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_2;
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::OutputSliceTooSmall;
    let mut decodesliceerror_1_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_1;
    let mut decodesliceerror_2: decode::DecodeSliceError = crate::decode::DecodeSliceError::clone(decodesliceerror_1_ref_0);
    let mut bool_2: bool = crate::engine::general_purpose::GeneralPurposeConfig::encode_padding(generalpurposeconfig_2_ref_0);
    let mut decodesliceerror_3: decode::DecodeSliceError = crate::decode::DecodeSliceError::OutputSliceTooSmall;
    let mut bool_3: bool = crate::engine::general_purpose::GeneralPurposeConfig::encode_padding(generalpurposeconfig_0_ref_0);
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata::new(usize_3, option_0);
    let mut bool_4: bool = crate::alphabet::ParseAlphabetError::eq(parsealphabeterror_1_ref_0, parsealphabeterror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_27() {
    rusty_monitor::set_test_id(27);
    let mut u8_0: u8 = 8u8;
    let mut usize_0: usize = 5604usize;
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireNone;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::new();
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_padding_mode(generalpurposeconfig_0, decodepaddingmode_0);
    let mut generalpurposeconfig_1_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_1;
    let mut u8_1: u8 = 72u8;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::UnprintableByte(u8_1);
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut u8_2: u8 = 39u8;
    let mut usize_1: usize = 5146usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_1, u8_2);
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_0);
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut u8_3: u8 = 12u8;
    let mut usize_2: usize = 5500usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_2, u8_3);
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_1);
    let mut decodesliceerror_1_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_1;
    let mut bool_0: bool = true;
    let mut usize_3: usize = 748usize;
    let mut option_0: std::option::Option<usize> = crate::encode::encoded_len(usize_3, bool_0);
    let mut usize_4: usize = 7601usize;
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata::new(usize_4, option_0);
    let mut decodemetadata_0_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_0;
    let mut u8_4: u8 = 31u8;
    let mut parsealphabeterror_1: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::ReservedByte(u8_4);
    let mut tuple_0: () = crate::engine::DecodeMetadata::assert_receiver_is_total_eq(decodemetadata_0_ref_0);
    let mut bool_1: bool = crate::decode::DecodeSliceError::eq(decodesliceerror_1_ref_0, decodesliceerror_0_ref_0);
    let mut parsealphabeterror_1_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_1;
    let mut bool_2: bool = crate::alphabet::ParseAlphabetError::eq(parsealphabeterror_1_ref_0, parsealphabeterror_0_ref_0);
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_0, u8_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_28() {
    rusty_monitor::set_test_id(28);
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireNone;
    let mut decodepaddingmode_0_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_0;
    let mut usize_0: usize = 2061usize;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::InvalidLength;
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut bool_0: bool = true;
    let mut usize_1: usize = 2273usize;
    let mut option_0: std::option::Option<usize> = crate::encode::encoded_len(usize_1, bool_0);
    let mut usize_2: usize = 2210usize;
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata::new(usize_2, option_0);
    let mut decodemetadata_0_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_0;
    let mut decodepaddingmode_1: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireCanonical;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_padding_mode(generalpurposeconfig_0, decodepaddingmode_1);
    let mut generalpurposeconfig_1_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_1;
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut u8_0: u8 = 100u8;
    let mut u8_1: u8 = 71u8;
    let mut usize_3: usize = 7737usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_3, u8_1);
    let mut decodeerror_0_ref_0: &decode::DecodeError = &mut decodeerror_0;
    let mut decodepaddingmode_2: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireNone;
    let mut parsealphabeterror_1: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::DuplicatedByte(u8_0);
    let mut tuple_0: () = crate::engine::DecodeMetadata::assert_receiver_is_total_eq(decodemetadata_0_ref_0);
    let mut decodepaddingmode_2_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_2;
    let mut tuple_1: () = crate::engine::DecodePaddingMode::assert_receiver_is_total_eq(decodepaddingmode_2_ref_0);
    let mut parsealphabeterror_1_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_1;
    let mut bool_1: bool = crate::alphabet::ParseAlphabetError::eq(parsealphabeterror_1_ref_0, parsealphabeterror_0_ref_0);
    let mut decodepaddingmode_3: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut decodepaddingmode_4: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::clone(decodepaddingmode_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_31() {
    rusty_monitor::set_test_id(31);
    let mut u8_0: u8 = 54u8;
    let mut usize_0: usize = 5369usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_0, u8_0);
    let mut decodeerror_0_ref_0: &decode::DecodeError = &mut decodeerror_0;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodeerror_1_ref_0: &decode::DecodeError = &mut decodeerror_1;
    let mut u8_1: u8 = 22u8;
    let mut usize_1: usize = 5830usize;
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_1, u8_1);
    let mut bool_0: bool = false;
    let mut usize_2: usize = 7443usize;
    let mut option_0: std::option::Option<usize> = crate::encode::encoded_len(usize_2, bool_0);
    let mut usize_3: usize = 648usize;
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut decodepaddingmode_0_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_0;
    let mut usize_4: usize = 2896usize;
    let mut decodepaddingmode_1: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireNone;
    let mut decodepaddingmode_1_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_1;
    let mut u8_2: u8 = 54u8;
    let mut usize_5: usize = 9716usize;
    let mut u8_3: u8 = 46u8;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::ReservedByte(u8_3);
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut usize_6: usize = 5212usize;
    let mut usize_7: usize = crate::decode::decoded_len_estimate(usize_6);
    let mut decodeerror_3: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_5, u8_2);
    let mut decodeerror_3_ref_0: &decode::DecodeError = &mut decodeerror_3;
    let mut tuple_0: () = crate::engine::DecodePaddingMode::assert_receiver_is_total_eq(decodepaddingmode_1_ref_0);
    let mut decodepaddingmode_2: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::clone(decodepaddingmode_0_ref_0);
    let mut decodepaddingmode_2_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_2;
    let mut tuple_1: () = crate::engine::DecodePaddingMode::assert_receiver_is_total_eq(decodepaddingmode_2_ref_0);
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata {decoded_len: usize_3, padding_offset: option_0};
    let mut decodepaddingmode_3: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut decodepaddingmode_3_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_3;
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_2);
    let mut bool_1: bool = crate::decode::DecodeError::eq(decodeerror_1_ref_0, decodeerror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_32() {
    rusty_monitor::set_test_id(32);
    let mut u8_0: u8 = 13u8;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::ReservedByte(u8_0);
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut bool_0: bool = false;
    let mut usize_0: usize = 1490usize;
    let mut option_0: std::option::Option<usize> = crate::encode::encoded_len(usize_0, bool_0);
    let mut usize_1: usize = 5060usize;
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata::new(usize_1, option_0);
    let mut decodemetadata_0_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_0;
    let mut bool_1: bool = false;
    let mut usize_2: usize = 4042usize;
    let mut option_1: std::option::Option<usize> = crate::encode::encoded_len(usize_2, bool_1);
    let mut usize_3: usize = 7419usize;
    let mut decodemetadata_1: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata {decoded_len: usize_3, padding_offset: option_1};
    let mut decodemetadata_1_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_1;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    let mut generalpurposeconfig_0_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_0;
    let mut usize_4: usize = 9274usize;
    let mut usize_5: usize = 1880usize;
    let mut option_2: std::option::Option<usize> = std::option::Option::Some(usize_5);
    let mut usize_6: usize = 5250usize;
    let mut decodemetadata_2: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata::new(usize_6, option_2);
    let mut decodemetadata_2_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_2;
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::OutputSliceTooSmall;
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut tuple_0: () = crate::decode::DecodeSliceError::assert_receiver_is_total_eq(decodesliceerror_0_ref_0);
    let mut tuple_1: () = crate::engine::DecodeMetadata::assert_receiver_is_total_eq(decodemetadata_2_ref_0);
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireNone;
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::clone(generalpurposeconfig_0_ref_0);
    let mut bool_2: bool = crate::engine::DecodeMetadata::eq(decodemetadata_1_ref_0, decodemetadata_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_36() {
    rusty_monitor::set_test_id(36);
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut usize_0: usize = 3931usize;
    let mut usize_1: usize = 5628usize;
    let mut u8_0: u8 = 39u8;
    let mut usize_2: usize = 5454usize;
    let mut bool_0: bool = true;
    let mut usize_3: usize = 4917usize;
    let mut option_1: std::option::Option<usize> = crate::encode::encoded_len(usize_3, bool_0);
    let mut usize_4: usize = 8256usize;
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata::new(usize_4, option_1);
    let mut decodemetadata_0_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_0;
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut usize_5: usize = 9578usize;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::InvalidLength;
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut bool_1: bool = true;
    let mut usize_6: usize = 1096usize;
    let mut option_2: std::option::Option<usize> = crate::encode::encoded_len(usize_6, bool_1);
    let mut usize_7: usize = 5052usize;
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::new();
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_padding_mode(generalpurposeconfig_0, decodepaddingmode_0);
    let mut generalpurposeconfig_1_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_1;
    let mut generalpurposeconfig_2: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::clone(generalpurposeconfig_1_ref_0);
    let mut decodemetadata_1: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata {decoded_len: usize_7, padding_offset: option_2};
    let mut decodemetadata_1_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_1;
    let mut usize_8: usize = crate::decode::decoded_len_estimate(usize_5);
    let mut tuple_0: () = crate::encode::EncodeSliceError::assert_receiver_is_total_eq(encodesliceerror_0_ref_0);
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_2, u8_0);
    let mut decodepaddingmode_1: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireNone;
    let mut decodemetadata_2: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata::new(usize_0, option_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_41() {
    rusty_monitor::set_test_id(41);
    let mut usize_0: usize = 645usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut usize_1: usize = 6244usize;
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::InvalidLength;
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut usize_2: usize = 816usize;
    let mut option_1: std::option::Option<usize> = std::option::Option::Some(usize_2);
    let mut usize_3: usize = 2406usize;
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata::new(usize_3, option_1);
    let mut decodemetadata_0_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_0;
    let mut bool_0: bool = true;
    let mut usize_4: usize = 6042usize;
    let mut option_2: std::option::Option<usize> = crate::encode::encoded_len(usize_4, bool_0);
    let mut usize_5: usize = 5836usize;
    let mut decodemetadata_1: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata {decoded_len: usize_5, padding_offset: option_2};
    let mut decodemetadata_1_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_1;
    let mut usize_6: usize = 7215usize;
    let mut usize_7: usize = crate::decode::decoded_len_estimate(usize_6);
    let mut encodesliceerror_1: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut bool_1: bool = crate::engine::DecodeMetadata::eq(decodemetadata_1_ref_0, decodemetadata_0_ref_0);
    let mut encodesliceerror_1_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_1;
    let mut tuple_0: () = crate::alphabet::ParseAlphabetError::assert_receiver_is_total_eq(parsealphabeterror_0_ref_0);
    let mut encodesliceerror_2: encode::EncodeSliceError = crate::encode::EncodeSliceError::clone(encodesliceerror_0_ref_0);
    let mut decodemetadata_2: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata::new(usize_1, option_0);
    let mut decodemetadata_2_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_2;
    let mut tuple_1: () = crate::engine::DecodeMetadata::assert_receiver_is_total_eq(decodemetadata_2_ref_0);
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireNone;
    let mut encodesliceerror_2_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_2;
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_43() {
    rusty_monitor::set_test_id(43);
    let mut u8_0: u8 = 23u8;
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::UnprintableByte(u8_0);
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut bool_0: bool = false;
    let mut usize_0: usize = 4228usize;
    let mut option_0: std::option::Option<usize> = crate::encode::encoded_len(usize_0, bool_0);
    let mut usize_1: usize = 3896usize;
    let mut u8_1: u8 = 102u8;
    let mut usize_2: usize = 2771usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_2, u8_1);
    let mut decodeerror_0_ref_0: &decode::DecodeError = &mut decodeerror_0;
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::OutputSliceTooSmall;
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut u8_2: u8 = 99u8;
    let mut usize_3: usize = 2552usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_3, u8_2);
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_1);
    let mut decodesliceerror_1_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_1;
    let mut u8_3: u8 = 13u8;
    let mut usize_4: usize = 4125usize;
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_4, u8_3);
    let mut decodeerror_2_ref_0: &decode::DecodeError = &mut decodeerror_2;
    let mut decodeerror_3: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    let mut generalpurposeconfig_0_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_0;
    let mut decodesliceerror_2: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_3);
    let mut decodeerror_4: decode::DecodeError = crate::decode::DecodeError::clone(decodeerror_2_ref_0);
    let mut decodesliceerror_2_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_2;
    let mut bool_1: bool = crate::decode::DecodeSliceError::eq(decodesliceerror_2_ref_0, decodesliceerror_1_ref_0);
    let mut tuple_0: () = crate::decode::DecodeSliceError::assert_receiver_is_total_eq(decodesliceerror_0_ref_0);
    let mut decodeerror_5: decode::DecodeError = crate::decode::DecodeError::clone(decodeerror_0_ref_0);
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata::new(usize_1, option_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_44() {
    rusty_monitor::set_test_id(44);
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    let mut generalpurposeconfig_0_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_0;
    let mut u8_0: u8 = 71u8;
    let mut usize_0: usize = 6538usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_0, u8_0);
    let mut decodeerror_0_ref_0: &decode::DecodeError = &mut decodeerror_0;
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireNone;
    let mut decodepaddingmode_0_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_0;
    let mut bool_0: bool = true;
    let mut usize_1: usize = 274usize;
    let mut option_0: std::option::Option<usize> = crate::encode::encoded_len(usize_1, bool_0);
    let mut usize_2: usize = 5322usize;
    let mut usize_3: usize = 9148usize;
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    let mut generalpurposeconfig_1_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_1;
    let mut decodepaddingmode_1: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut decodepaddingmode_1_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_1;
    let mut usize_4: usize = 3749usize;
    let mut decodepaddingmode_2: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut decodepaddingmode_2_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_2;
    let mut decodepaddingmode_3: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::clone(decodepaddingmode_2_ref_0);
    let mut tuple_0: () = crate::engine::DecodePaddingMode::assert_receiver_is_total_eq(decodepaddingmode_1_ref_0);
    let mut bool_1: bool = crate::engine::general_purpose::GeneralPurposeConfig::encode_padding(generalpurposeconfig_1_ref_0);
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::InvalidLength;
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata {decoded_len: usize_2, padding_offset: option_0};
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut decodepaddingmode_3_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_3;
    let mut bool_2: bool = crate::engine::DecodePaddingMode::eq(decodepaddingmode_3_ref_0, decodepaddingmode_0_ref_0);
    let mut tuple_1: () = crate::decode::DecodeError::assert_receiver_is_total_eq(decodeerror_0_ref_0);
    let mut generalpurposeconfig_2: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::clone(generalpurposeconfig_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_45() {
    rusty_monitor::set_test_id(45);
    let mut bool_0: bool = false;
    let mut usize_0: usize = 454usize;
    let mut option_0: std::option::Option<usize> = crate::encode::encoded_len(usize_0, bool_0);
    let mut usize_1: usize = 8925usize;
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata::new(usize_1, option_0);
    let mut decodemetadata_0_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_0;
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut encodesliceerror_1: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_1_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_1;
    let mut usize_2: usize = 2689usize;
    let mut u8_0: u8 = 34u8;
    let mut usize_3: usize = 7218usize;
    let mut encodesliceerror_2: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_2_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_2;
    let mut bool_1: bool = true;
    let mut usize_4: usize = 1143usize;
    let mut option_1: std::option::Option<usize> = crate::encode::encoded_len(usize_4, bool_1);
    let mut usize_5: usize = 726usize;
    let mut encodesliceerror_3: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_3_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_3;
    let mut tuple_0: () = crate::encode::EncodeSliceError::assert_receiver_is_total_eq(encodesliceerror_3_ref_0);
    let mut encodesliceerror_4: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut decodemetadata_1: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata {decoded_len: usize_5, padding_offset: option_1};
    let mut decodemetadata_1_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_1;
    let mut tuple_1: () = crate::engine::DecodeMetadata::assert_receiver_is_total_eq(decodemetadata_1_ref_0);
    let mut encodesliceerror_4_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_4;
    let mut bool_2: bool = crate::encode::EncodeSliceError::eq(encodesliceerror_4_ref_0, encodesliceerror_2_ref_0);
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_3, u8_0);
    let mut decodeerror_0_ref_0: &decode::DecodeError = &mut decodeerror_0;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut bool_3: bool = crate::encode::EncodeSliceError::eq(encodesliceerror_1_ref_0, encodesliceerror_0_ref_0);
    let mut decodeerror_1_ref_0: &decode::DecodeError = &mut decodeerror_1;
    let mut tuple_2: () = crate::engine::DecodeMetadata::assert_receiver_is_total_eq(decodemetadata_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_46() {
    rusty_monitor::set_test_id(46);
    let mut u8_0: u8 = 58u8;
    let mut usize_0: usize = 801usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut usize_1: usize = 1590usize;
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata {decoded_len: usize_1, padding_offset: option_0};
    let mut decodemetadata_0_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_0;
    let mut bool_0: bool = true;
    let mut usize_2: usize = 876usize;
    let mut option_1: std::option::Option<usize> = crate::encode::encoded_len(usize_2, bool_0);
    let mut usize_3: usize = 3635usize;
    let mut decodemetadata_1: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata {decoded_len: usize_3, padding_offset: option_1};
    let mut decodemetadata_1_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_1;
    let mut usize_4: usize = 6379usize;
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireCanonical;
    let mut decodepaddingmode_0_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_0;
    let mut u8_1: u8 = 119u8;
    let mut usize_5: usize = 3224usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_5, u8_1);
    let mut decodeerror_0_ref_0: &decode::DecodeError = &mut decodeerror_0;
    let mut u8_2: u8 = 111u8;
    let mut usize_6: usize = 9202usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_6, u8_2);
    let mut decodeerror_1_ref_0: &decode::DecodeError = &mut decodeerror_1;
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::clone(decodeerror_0_ref_0);
    let mut decodeerror_2_ref_0: &decode::DecodeError = &mut decodeerror_2;
    let mut tuple_0: () = crate::decode::DecodeError::assert_receiver_is_total_eq(decodeerror_2_ref_0);
    let mut tuple_1: () = crate::engine::DecodePaddingMode::assert_receiver_is_total_eq(decodepaddingmode_0_ref_0);
    let mut decodepaddingmode_1: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut bool_1: bool = crate::engine::DecodeMetadata::eq(decodemetadata_1_ref_0, decodemetadata_0_ref_0);
    let mut decodeerror_3: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_0, u8_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_48() {
    rusty_monitor::set_test_id(48);
    let mut u8_0: u8 = 44u8;
    let mut usize_0: usize = 5804usize;
    let mut u8_1: u8 = 31u8;
    let mut usize_1: usize = 178usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_1, u8_1);
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut option_0: std::option::Option<usize> = std::option::Option::None;
    let mut usize_2: usize = 7093usize;
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata {decoded_len: usize_2, padding_offset: option_0};
    let mut decodemetadata_0_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_0;
    let mut u8_2: u8 = 124u8;
    let mut u8_3: u8 = 31u8;
    let mut usize_3: usize = 4652usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_3, u8_3);
    let mut decodeerror_1_ref_0: &decode::DecodeError = &mut decodeerror_1;
    let mut bool_0: bool = true;
    let mut usize_4: usize = 6685usize;
    let mut option_1: std::option::Option<usize> = crate::encode::encoded_len(usize_4, bool_0);
    let mut usize_5: usize = 102usize;
    let mut usize_6: usize = 6241usize;
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_6);
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_2);
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut decodemetadata_1: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata::new(usize_5, option_1);
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::UnprintableByte(u8_2);
    let mut decodemetadata_1_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_1;
    let mut bool_1: bool = crate::engine::DecodeMetadata::eq(decodemetadata_1_ref_0, decodemetadata_0_ref_0);
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_0);
    let mut decodesliceerror_1_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_1;
    let mut decodeerror_3: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_0, u8_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_49() {
    rusty_monitor::set_test_id(49);
    let mut usize_0: usize = 8971usize;
    let mut option_0: std::option::Option<usize> = std::option::Option::Some(usize_0);
    let mut usize_1: usize = 8606usize;
    let mut decodemetadata_0: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata {decoded_len: usize_1, padding_offset: option_0};
    let mut decodemetadata_0_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_0;
    let mut usize_2: usize = 4381usize;
    let mut option_1: std::option::Option<usize> = std::option::Option::Some(usize_2);
    let mut usize_3: usize = 4295usize;
    let mut decodemetadata_1: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata {decoded_len: usize_3, padding_offset: option_1};
    let mut decodemetadata_1_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_1;
    let mut usize_4: usize = 9678usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_4);
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_0);
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut u8_0: u8 = 122u8;
    let mut bool_0: bool = true;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::new();
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_encode_padding(generalpurposeconfig_0, bool_0);
    let mut generalpurposeconfig_1_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_1;
    let mut u8_1: u8 = 113u8;
    let mut usize_5: usize = 2452usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_5, u8_1);
    let mut generalpurposeconfig_2: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    let mut generalpurposeconfig_2_ref_0: &crate::engine::general_purpose::GeneralPurposeConfig = &mut generalpurposeconfig_2;
    let mut usize_6: usize = 4674usize;
    let mut option_2: std::option::Option<usize> = std::option::Option::Some(usize_6);
    let mut usize_7: usize = 3870usize;
    let mut decodemetadata_2: crate::engine::DecodeMetadata = crate::engine::DecodeMetadata::new(usize_7, option_2);
    let mut decodemetadata_2_ref_0: &crate::engine::DecodeMetadata = &mut decodemetadata_2;
    let mut bool_1: bool = crate::engine::general_purpose::GeneralPurposeConfig::encode_padding(generalpurposeconfig_2_ref_0);
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_1);
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::UnprintableByte(u8_0);
    let mut decodesliceerror_1_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_1;
    let mut tuple_0: () = crate::decode::DecodeSliceError::assert_receiver_is_total_eq(decodesliceerror_1_ref_0);
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut tuple_1: () = crate::alphabet::ParseAlphabetError::assert_receiver_is_total_eq(parsealphabeterror_0_ref_0);
    let mut tuple_2: () = crate::decode::DecodeSliceError::assert_receiver_is_total_eq(decodesliceerror_0_ref_0);
    let mut bool_2: bool = crate::engine::DecodeMetadata::eq(decodemetadata_1_ref_0, decodemetadata_0_ref_0);
    panic!("From RustyUnit with love");
}
}
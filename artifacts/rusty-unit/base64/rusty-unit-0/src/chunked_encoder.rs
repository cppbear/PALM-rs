use crate::{
    encode::add_padding,
    engine::{Config, Engine},
};
#[cfg(any(feature = "alloc", test))]
use alloc::string::String;
#[cfg(any(feature = "alloc", test))]
use core::str;

/// The output mechanism for ChunkedEncoder's encoded bytes.
pub trait Sink {
    type Error;

    /// Handle a chunk of encoded base64 data (as UTF-8 bytes)
    fn write_encoded_bytes(&mut self, encoded: &[u8]) -> Result<(), Self::Error>;
}

/// A base64 encoder that emits encoded bytes in chunks without heap allocation.
pub struct ChunkedEncoder<'e, E: Engine + ?Sized> {
    engine: &'e E,
}

impl<'e, E: Engine + ?Sized> ChunkedEncoder<'e, E> {
    pub fn new(engine: &'e E) -> ChunkedEncoder<'e, E> {
        ChunkedEncoder { engine }
    }

    pub fn encode<S: Sink>(&self, bytes: &[u8], sink: &mut S) -> Result<(), S::Error> {
        const BUF_SIZE: usize = 1024;
        const CHUNK_SIZE: usize = BUF_SIZE / 4 * 3;

        let mut buf = [0; BUF_SIZE];
        for chunk in bytes.chunks(CHUNK_SIZE) {
            let mut len = self.engine.internal_encode(chunk, &mut buf);
            if chunk.len() != CHUNK_SIZE && self.engine.config().encode_padding() {
                // Final, potentially partial, chunk.
                // Only need to consider if padding is needed on a partial chunk since full chunk
                // is a multiple of 3, which therefore won't be padded.
                // Pad output to multiple of four bytes if required by config.
                len += add_padding(len, &mut buf[len..]);
            }
            sink.write_encoded_bytes(&buf[..len])?;
        }

        Ok(())
    }
}

// A really simple sink that just appends to a string
#[cfg(any(feature = "alloc", test))]
pub(crate) struct StringSink<'a> {
    string: &'a mut String,
}

#[cfg(any(feature = "alloc", test))]
impl<'a> StringSink<'a> {
    pub(crate) fn new(s: &mut String) -> StringSink {
        StringSink { string: s }
    }
}

#[cfg(any(feature = "alloc", test))]
impl<'a> Sink for StringSink<'a> {
    type Error = ();

    fn write_encoded_bytes(&mut self, s: &[u8]) -> Result<(), Self::Error> {
        self.string.push_str(str::from_utf8(s).unwrap());

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use rand::{
        distributions::{Distribution, Uniform},
        Rng, SeedableRng,
    };

    use crate::{
        alphabet::STANDARD,
        engine::general_purpose::{GeneralPurpose, GeneralPurposeConfig, PAD},
        tests::random_engine,
    };

    use super::*;

    #[test]
    fn chunked_encode_empty() {
        assert_eq!("", chunked_encode_str(&[], PAD));
    }

    #[test]
    fn chunked_encode_intermediate_fast_loop() {
        // > 8 bytes input, will enter the pretty fast loop
        assert_eq!("Zm9vYmFyYmF6cXV4", chunked_encode_str(b"foobarbazqux", PAD));
    }

    #[test]
    fn chunked_encode_fast_loop() {
        // > 32 bytes input, will enter the uber fast loop
        assert_eq!(
            "Zm9vYmFyYmF6cXV4cXV1eGNvcmdlZ3JhdWx0Z2FycGx5eg==",
            chunked_encode_str(b"foobarbazquxquuxcorgegraultgarplyz", PAD)
        );
    }

    #[test]
    fn chunked_encode_slow_loop_only() {
        // < 8 bytes input, slow loop only
        assert_eq!("Zm9vYmFy", chunked_encode_str(b"foobar", PAD));
    }

    #[test]
    fn chunked_encode_matches_normal_encode_random_string_sink() {
        let helper = StringSinkTestHelper;
        chunked_encode_matches_normal_encode_random(&helper);
    }

    pub fn chunked_encode_matches_normal_encode_random<S: SinkTestHelper>(sink_test_helper: &S) {
        let mut input_buf: Vec<u8> = Vec::new();
        let mut output_buf = String::new();
        let mut rng = rand::rngs::SmallRng::from_entropy();
        let input_len_range = Uniform::new(1, 10_000);

        for _ in 0..20_000 {
            input_buf.clear();
            output_buf.clear();

            let buf_len = input_len_range.sample(&mut rng);
            for _ in 0..buf_len {
                input_buf.push(rng.gen());
            }

            let engine = random_engine(&mut rng);

            let chunk_encoded_string = sink_test_helper.encode_to_string(&engine, &input_buf);
            engine.encode_string(&input_buf, &mut output_buf);

            assert_eq!(output_buf, chunk_encoded_string, "input len={}", buf_len);
        }
    }

    fn chunked_encode_str(bytes: &[u8], config: GeneralPurposeConfig) -> String {
        let mut s = String::new();

        let mut sink = StringSink::new(&mut s);
        let engine = GeneralPurpose::new(&STANDARD, config);
        let encoder = ChunkedEncoder::new(&engine);
        encoder.encode(bytes, &mut sink).unwrap();

        s
    }

    // An abstraction around sinks so that we can have tests that easily to any sink implementation
    pub trait SinkTestHelper {
        fn encode_to_string<E: Engine>(&self, engine: &E, bytes: &[u8]) -> String;
    }

    struct StringSinkTestHelper;

    impl SinkTestHelper for StringSinkTestHelper {
        fn encode_to_string<E: Engine>(&self, engine: &E, bytes: &[u8]) -> String {
            let encoder = ChunkedEncoder::new(engine);
            let mut s = String::new();
            let mut sink = StringSink::new(&mut s);
            encoder.encode(bytes, &mut sink).unwrap();

            s
        }
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::clone::Clone;
	use std::cmp::PartialEq;
	use std::convert::From;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_20() {
    rusty_monitor::set_test_id(20);
    let mut u8_0: u8 = 98u8;
    let mut usize_0: usize = 3768usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_0, u8_0);
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireNone;
    let mut decodepaddingmode_0_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_0;
    let mut decodepaddingmode_1: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::Indifferent;
    let mut decodepaddingmode_1_ref_0: &engine::DecodePaddingMode = &mut decodepaddingmode_1;
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut usize_1: usize = 7741usize;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_1);
    let mut decodeerror_1_ref_0: &decode::DecodeError = &mut decodeerror_1;
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_2);
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut u8_1: u8 = 89u8;
    let mut usize_2: usize = 2248usize;
    let mut decodeerror_3: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_2, u8_1);
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_3);
    let mut decodesliceerror_1_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_1;
    let mut decodeerror_4: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodesliceerror_2: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_4);
    let mut decodesliceerror_2_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_2;
    let mut decodesliceerror_3: decode::DecodeSliceError = crate::decode::DecodeSliceError::clone(decodesliceerror_2_ref_0);
    let mut decodesliceerror_3_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_3;
    let mut bool_0: bool = crate::decode::DecodeSliceError::eq(decodesliceerror_3_ref_0, decodesliceerror_1_ref_0);
    let mut bool_1: bool = crate::engine::DecodePaddingMode::eq(decodepaddingmode_1_ref_0, decodepaddingmode_0_ref_0);
    let mut decodesliceerror_4: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_0);
    let mut decodesliceerror_4_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_4;
    let mut decodeerror_5: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    panic!("From RustyUnit with love");
}
}
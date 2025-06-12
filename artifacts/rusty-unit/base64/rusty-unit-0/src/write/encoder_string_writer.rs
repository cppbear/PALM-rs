use super::encoder::EncoderWriter;
use crate::engine::Engine;
use std::io;

/// A `Write` implementation that base64-encodes data using the provided config and accumulates the
/// resulting base64 utf8 `&str` in a [StrConsumer] implementation (typically `String`), which is
/// then exposed via `into_inner()`.
///
/// # Examples
///
/// Buffer base64 in a new String:
///
/// ```
/// use std::io::Write;
/// use base64::engine::general_purpose;
///
/// let mut enc = base64::write::EncoderStringWriter::new(&general_purpose::STANDARD);
///
/// enc.write_all(b"asdf").unwrap();
///
/// // get the resulting String
/// let b64_string = enc.into_inner();
///
/// assert_eq!("YXNkZg==", &b64_string);
/// ```
///
/// Or, append to an existing `String`, which implements `StrConsumer`:
///
/// ```
/// use std::io::Write;
/// use base64::engine::general_purpose;
///
/// let mut buf = String::from("base64: ");
///
/// let mut enc = base64::write::EncoderStringWriter::from_consumer(
///     &mut buf,
///     &general_purpose::STANDARD);
///
/// enc.write_all(b"asdf").unwrap();
///
/// // release the &mut reference on buf
/// let _ = enc.into_inner();
///
/// assert_eq!("base64: YXNkZg==", &buf);
/// ```
///
/// # Performance
///
/// Because it has to validate that the base64 is UTF-8, it is about 80% as fast as writing plain
/// bytes to a `io::Write`.
pub struct EncoderStringWriter<'e, E: Engine, S: StrConsumer> {
    encoder: EncoderWriter<'e, E, Utf8SingleCodeUnitWriter<S>>,
}

impl<'e, E: Engine, S: StrConsumer> EncoderStringWriter<'e, E, S> {
    /// Create a EncoderStringWriter that will append to the provided `StrConsumer`.
    pub fn from_consumer(str_consumer: S, engine: &'e E) -> Self {
        EncoderStringWriter {
            encoder: EncoderWriter::new(Utf8SingleCodeUnitWriter { str_consumer }, engine),
        }
    }

    /// Encode all remaining buffered data, including any trailing incomplete input triples and
    /// associated padding.
    ///
    /// Returns the base64-encoded form of the accumulated written data.
    pub fn into_inner(mut self) -> S {
        self.encoder
            .finish()
            .expect("Writing to a consumer should never fail")
            .str_consumer
    }
}

impl<'e, E: Engine> EncoderStringWriter<'e, E, String> {
    /// Create a EncoderStringWriter that will encode into a new `String` with the provided config.
    pub fn new(engine: &'e E) -> Self {
        EncoderStringWriter::from_consumer(String::new(), engine)
    }
}

impl<'e, E: Engine, S: StrConsumer> io::Write for EncoderStringWriter<'e, E, S> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.encoder.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.encoder.flush()
    }
}

/// An abstraction around consuming `str`s produced by base64 encoding.
pub trait StrConsumer {
    /// Consume the base64 encoded data in `buf`
    fn consume(&mut self, buf: &str);
}

/// As for io::Write, `StrConsumer` is implemented automatically for `&mut S`.
impl<S: StrConsumer + ?Sized> StrConsumer for &mut S {
    fn consume(&mut self, buf: &str) {
        (**self).consume(buf);
    }
}

/// Pushes the str onto the end of the String
impl StrConsumer for String {
    fn consume(&mut self, buf: &str) {
        self.push_str(buf);
    }
}

/// A `Write` that only can handle bytes that are valid single-byte UTF-8 code units.
///
/// This is safe because we only use it when writing base64, which is always valid UTF-8.
struct Utf8SingleCodeUnitWriter<S: StrConsumer> {
    str_consumer: S,
}

impl<S: StrConsumer> io::Write for Utf8SingleCodeUnitWriter<S> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Because we expect all input to be valid utf-8 individual bytes, we can encode any buffer
        // length
        let s = std::str::from_utf8(buf).expect("Input must be valid UTF-8");

        self.str_consumer.consume(s);

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        // no op
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        engine::Engine, tests::random_engine, write::encoder_string_writer::EncoderStringWriter,
    };
    use rand::Rng;
    use std::cmp;
    use std::io::Write;

    #[test]
    fn every_possible_split_of_input() {
        let mut rng = rand::thread_rng();
        let mut orig_data = Vec::<u8>::new();
        let mut normal_encoded = String::new();

        let size = 5_000;

        for i in 0..size {
            orig_data.clear();
            normal_encoded.clear();

            orig_data.resize(size, 0);
            rng.fill(&mut orig_data[..]);

            let engine = random_engine(&mut rng);
            engine.encode_string(&orig_data, &mut normal_encoded);

            let mut stream_encoder = EncoderStringWriter::new(&engine);
            // Write the first i bytes, then the rest
            stream_encoder.write_all(&orig_data[0..i]).unwrap();
            stream_encoder.write_all(&orig_data[i..]).unwrap();

            let stream_encoded = stream_encoder.into_inner();

            assert_eq!(normal_encoded, stream_encoded);
        }
    }
    #[test]
    fn incremental_writes() {
        let mut rng = rand::thread_rng();
        let mut orig_data = Vec::<u8>::new();
        let mut normal_encoded = String::new();

        let size = 5_000;

        for _ in 0..size {
            orig_data.clear();
            normal_encoded.clear();

            orig_data.resize(size, 0);
            rng.fill(&mut orig_data[..]);

            let engine = random_engine(&mut rng);
            engine.encode_string(&orig_data, &mut normal_encoded);

            let mut stream_encoder = EncoderStringWriter::new(&engine);
            // write small nibbles of data
            let mut offset = 0;
            while offset < size {
                let nibble_size = cmp::min(rng.gen_range(0..=64), size - offset);
                let len = stream_encoder
                    .write(&orig_data[offset..offset + nibble_size])
                    .unwrap();
                offset += len;
            }

            let stream_encoded = stream_encoder.into_inner();

            assert_eq!(normal_encoded, stream_encoded);
        }
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::default::Default;
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::Eq;
	use std::convert::From;
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3() {
    rusty_monitor::set_test_id(3);
    let mut parsealphabeterror_0: alphabet::ParseAlphabetError = crate::alphabet::ParseAlphabetError::InvalidLength;
    let mut parsealphabeterror_0_ref_0: &alphabet::ParseAlphabetError = &mut parsealphabeterror_0;
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut u8_0: u8 = 3u8;
    let mut usize_0: usize = 1447usize;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_0, u8_0);
    let mut decodeerror_0_ref_0: &decode::DecodeError = &mut decodeerror_0;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodeerror_1_ref_0: &decode::DecodeError = &mut decodeerror_1;
    let mut usize_1: usize = 2718usize;
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_1);
    let mut decodeerror_2_ref_0: &decode::DecodeError = &mut decodeerror_2;
    let mut encodesliceerror_1: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_1_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_1;
    let mut usize_2: usize = 535usize;
    let mut encodesliceerror_2: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_2_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_2;
    let mut encodesliceerror_3: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_3_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_3;
    let mut bool_0: bool = crate::encode::EncodeSliceError::eq(encodesliceerror_3_ref_0, encodesliceerror_2_ref_0);
    let mut decodeerror_3: decode::DecodeError = crate::decode::DecodeError::InvalidLength(usize_2);
    let mut decodeerror_4: decode::DecodeError = crate::decode::DecodeError::clone(decodeerror_2_ref_0);
    let mut decodeerror_4_ref_0: &decode::DecodeError = &mut decodeerror_4;
    let mut bool_1: bool = crate::decode::DecodeError::eq(decodeerror_4_ref_0, decodeerror_1_ref_0);
    let mut decodeerror_3_ref_0: &decode::DecodeError = &mut decodeerror_3;
    let mut bool_2: bool = crate::decode::DecodeError::eq(decodeerror_3_ref_0, decodeerror_0_ref_0);
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::OutputSliceTooSmall;
    let mut tuple_0: () = crate::alphabet::ParseAlphabetError::assert_receiver_is_total_eq(parsealphabeterror_0_ref_0);
    panic!("From RustyUnit with love");
}

#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_26() {
    rusty_monitor::set_test_id(26);
    let mut encodesliceerror_0: encode::EncodeSliceError = crate::encode::EncodeSliceError::OutputSliceTooSmall;
    let mut encodesliceerror_0_ref_0: &encode::EncodeSliceError = &mut encodesliceerror_0;
    let mut decodeerror_0: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodesliceerror_0: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_0);
    let mut decodesliceerror_0_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_0;
    let mut decodeerror_1: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodeerror_1_ref_0: &decode::DecodeError = &mut decodeerror_1;
    let mut decodeerror_2: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodeerror_2_ref_0: &decode::DecodeError = &mut decodeerror_2;
    let mut u8_0: u8 = 11u8;
    let mut usize_0: usize = 6548usize;
    let mut decodepaddingmode_0: engine::DecodePaddingMode = crate::engine::DecodePaddingMode::RequireCanonical;
    let mut bool_0: bool = true;
    let mut generalpurposeconfig_0: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::default();
    let mut generalpurposeconfig_1: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_encode_padding(generalpurposeconfig_0, bool_0);
    let mut u8_1: u8 = 17u8;
    let mut usize_1: usize = 4682usize;
    let mut decodeerror_3: decode::DecodeError = crate::decode::DecodeError::InvalidLastSymbol(usize_1, u8_1);
    let mut decodesliceerror_1: decode::DecodeSliceError = crate::decode::DecodeSliceError::from(decodeerror_3);
    let mut decodesliceerror_1_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_1;
    let mut decodeerror_4: decode::DecodeError = crate::decode::DecodeError::InvalidPadding;
    let mut decodesliceerror_2: decode::DecodeSliceError = crate::decode::DecodeSliceError::DecodeError(decodeerror_4);
    let mut decodesliceerror_2_ref_0: &decode::DecodeSliceError = &mut decodesliceerror_2;
    let mut generalpurposeconfig_2: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::new();
    let mut bool_1: bool = crate::decode::DecodeSliceError::eq(decodesliceerror_2_ref_0, decodesliceerror_1_ref_0);
    let mut generalpurposeconfig_3: crate::engine::general_purpose::GeneralPurposeConfig = crate::engine::general_purpose::GeneralPurposeConfig::with_decode_padding_mode(generalpurposeconfig_1, decodepaddingmode_0);
    let mut decodeerror_5: decode::DecodeError = crate::decode::DecodeError::InvalidByte(usize_0, u8_0);
    let mut decodeerror_5_ref_0: &decode::DecodeError = &mut decodeerror_5;
    let mut bool_2: bool = crate::decode::DecodeError::eq(decodeerror_5_ref_0, decodeerror_2_ref_0);
    let mut decodeerror_6: decode::DecodeError = crate::decode::DecodeError::clone(decodeerror_1_ref_0);
    let mut decodesliceerror_3: decode::DecodeSliceError = crate::decode::DecodeSliceError::clone(decodesliceerror_0_ref_0);
    panic!("From RustyUnit with love");
}
}
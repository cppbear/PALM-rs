// Answer 0

#[test]
fn test_finish_success() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            0
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata {})
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }
    
    struct TestWriter {
        written: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let engine = TestEngine;
    let writer = TestWriter { written: vec![] };

    let mut encoder = EncoderWriter::new(writer, &engine);
    encoder.finish().unwrap();
}

#[test]
#[should_panic(expected = "Encoder has already had finish() called")]
fn test_finish_after_finish_panics() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            0
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata {})
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    struct TestWriter {
        written: Vec<u8>,
    }

    impl io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let engine = TestEngine;
    let writer = TestWriter { written: vec![] };

    let mut encoder = EncoderWriter::new(writer, &engine);
    encoder.finish().unwrap();
    encoder.finish().unwrap(); // This should panic
}

#[test]
fn test_finish_with_error() {
    struct TestEngine;
    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            0
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata {})
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    struct FailingWriter;

    impl io::Write for FailingWriter {
        fn write(&mut self, _buf: &[u8]) -> Result<usize> {
            Err(io::Error::new(ErrorKind::Other, "Writing failed"))
        }

        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }

    let engine = TestEngine;
    let writer = FailingWriter;

    let mut encoder = EncoderWriter::new(writer, &engine);
    let result = encoder.finish();
    assert!(result.is_err());
}


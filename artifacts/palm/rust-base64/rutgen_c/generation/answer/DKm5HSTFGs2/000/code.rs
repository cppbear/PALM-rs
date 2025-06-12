// Answer 0

#[test]
fn test_finish_success() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let encoded_size = input.len(); // Simplified mock behavior
            output[..encoded_size].copy_from_slice(input);
            encoded_size
        }
        
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len // Simplified mock behavior
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata {}) // Simplified
        }

        fn config(&self) -> &Self::Config {
            &() // Simplified
        }
    }

    struct MockWriter {
        written: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.written.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mock_engine = MockEngine;
    let mock_writer = MockWriter { written: Vec::new() };
    let mut encoder_writer = EncoderWriter::new(mock_writer, &mock_engine);

    let result = encoder_writer.finish();
    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "Encoder has already had finish() called")]
fn test_finish_called_twice() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output.copy_from_slice(input);
            input.len()
        }
        
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
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

    struct MockWriter {
        written: Vec<u8>,
    }

    impl io::Write for MockWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Ok(0)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let mock_engine = MockEngine;
    let mock_writer = MockWriter { written: Vec::new() };
    let mut encoder_writer = EncoderWriter::new(mock_writer, &mock_engine);

    let _ = encoder_writer.finish();
    let _ = encoder_writer.finish(); // Should panic here
}


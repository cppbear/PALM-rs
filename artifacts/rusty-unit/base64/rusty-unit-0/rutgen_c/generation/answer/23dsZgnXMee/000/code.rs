// Answer 0

#[test]
fn test_encode_full_chunk() {
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = TestConfig;
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            // Dummy encoding for testing
            output[..input.len()].copy_from_slice(input);
            input.len()
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len // Just a placeholder
        }

        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &TestConfig
        }
    }

    struct TestConfig;

    impl Config for TestConfig {
        fn encode_padding(&self) -> bool {
            false
        }
    }

    struct TestSink {
        output: Vec<u8>,
    }

    impl TestSink {
        fn new() -> Self {
            Self { output: Vec::new() }
        }

        fn write_encoded_bytes(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
            self.output.extend_from_slice(bytes);
            Ok(())
        }
    }

    let engine = TestEngine;
    let encoder = ChunkedEncoder::new(&engine);
    let mut sink = TestSink::new();

    let data = b"123456789012"; // 12 bytes, single full chunk
    let result = encoder.encode(data, &mut sink);
    
    assert!(result.is_ok());
    assert_eq!(sink.output, data);
}

#[test]
fn test_encode_partial_chunk_with_padding() {
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = TestConfig;
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..input.len()].copy_from_slice(input);
            input.len()
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }

        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &TestConfig
        }
    }

    struct TestConfig;

    impl Config for TestConfig {
        fn encode_padding(&self) -> bool {
            true
        }
    }

    struct TestSink {
        output: Vec<u8>,
    }

    impl TestSink {
        fn new() -> Self {
            Self { output: Vec::new() }
        }

        fn write_encoded_bytes(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
            self.output.extend_from_slice(bytes);
            Ok(())
        }
    }

    let engine = TestEngine;
    let encoder = ChunkedEncoder::new(&engine);
    let mut sink = TestSink::new();

    let data = b"1234567"; // 7 bytes, needs padding
    let result = encoder.encode(data, &mut sink);
    
    assert!(result.is_ok());
    assert_eq!(sink.output, b"1234567\x00\x00");
}


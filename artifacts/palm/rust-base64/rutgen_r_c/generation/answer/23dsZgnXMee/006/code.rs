// Answer 0

#[test]
fn test_encode_full_chunks() {
    struct MockEngine {
        config: MockConfig,
    }

    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = usize; // Dummy type for the test

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let encoded_len = input.len(); // For simplicity, not doing real encoding
            output[..encoded_len].copy_from_slice(input);
            encoded_len
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
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &self.config
        }
    }

    struct MockConfig {
        padding: bool,
    }

    impl Config for MockConfig {
        fn encode_padding(&self) -> bool {
            self.padding
        }
    }

    struct MockSink {
        data: Vec<u8>,
    }

    impl MockSink {
        fn new() -> Self {
            MockSink { data: Vec::new() }
        }

        fn write_encoded_bytes(&mut self, bytes: &[u8]) -> Result<(), String> {
            self.data.extend_from_slice(bytes);
            Ok(())
        }
    }

    let engine = MockEngine { config: MockConfig { padding: false } };
    let encoder = ChunkedEncoder::new(&engine);
    let input = vec![1u8; 1024]; // Fill the buffer with 1024 bytes
    let mut sink = MockSink::new();
    
    let result = encoder.encode(&input, &mut sink);
    
    assert!(result.is_ok());
    assert_eq!(sink.data.len(), 1024); // Ensure all input bytes were written
}

#[test]
fn test_encode_partial_chunk_with_padding() {
    struct MockEngine {
        config: MockConfig,
    }

    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let encoded_len = input.len(); 
            output[..encoded_len].copy_from_slice(input);
            encoded_len
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
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &self.config
        }
    }

    struct MockConfig {
        padding: bool,
    }

    impl Config for MockConfig {
        fn encode_padding(&self) -> bool {
            self.padding
        }
    }

    struct MockSink {
        data: Vec<u8>,
    }

    impl MockSink {
        fn new() -> Self {
            MockSink { data: Vec::new() }
        }

        fn write_encoded_bytes(&mut self, bytes: &[u8]) -> Result<(), String> {
            self.data.extend_from_slice(bytes);
            Ok(())
        }
    }

    let engine = MockEngine { config: MockConfig { padding: true } };
    let encoder = ChunkedEncoder::new(&engine);
    let input = vec![1u8; 1025]; // One more than a full chunk
    let mut sink = MockSink::new();
    
    let result = encoder.encode(&input, &mut sink);
    
    assert!(result.is_ok());
    assert!(!sink.data.is_empty()); // Ensure something was written
}


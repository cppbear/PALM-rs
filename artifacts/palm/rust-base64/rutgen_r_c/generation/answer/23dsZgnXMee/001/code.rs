// Answer 0

#[test]
fn test_encode_partial_chunk_with_padding() {
    struct MockSink {
        written: Vec<u8>,
        error: Option<&'static str>,
    }

    impl MockSink {
        fn new() -> Self {
            Self { written: Vec::new(), error: None }
        }
    }

    impl Sink for MockSink {
        type Error = &'static str;

        fn write_encoded_bytes(&mut self, bytes: &[u8]) -> Result<(), Self::Error> {
            if self.error.is_some() {
                Err(self.error.unwrap())
            } else {
                self.written.extend_from_slice(bytes);
                Ok(())
            }
        }
    }

    struct MockEngineConfig {
        padding: bool,
    }

    struct MockEngine {
        config: MockEngineConfig,
    }

    impl Config for MockEngineConfig {
        fn encode_padding(&self) -> bool {
            self.padding
        }
    }

    impl Engine for MockEngine {
        type Config = MockEngineConfig;
        type DecodeEstimate = ();

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            for (i, byte) in input.iter().enumerate() {
                if i < output.len() {
                    output[i] = *byte; // Mock encoding by simply copying
                }
            }
            input.len() // Returning length as number of bytes "encoded"
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            ()
        }

        fn internal_decode(
            &self,
            _: &[u8],
            _: &mut [u8],
            _: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &self.config
        }
    }

    let mock_config = MockEngineConfig { padding: true };
    let mock_engine = MockEngine { config: mock_config };
    let encoder = ChunkedEncoder::new(&mock_engine);
    let mut sink = MockSink::new();

    let input_data = vec![1, 2, 3, 4, 5]; // This will create a partial chunk
    encoder.encode(&input_data, &mut sink).unwrap();

    assert_eq!(sink.written.len(), 8); // Expected length with padding for 5 bytes
}

#[test]
#[should_panic]
fn test_encode_write_error() {
    struct MockSink {
        error: &'static str,
    }

    impl MockSink {
        fn new(error: &'static str) -> Self {
            Self { error }
        }
    }

    impl Sink for MockSink {
        type Error = &'static str;

        fn write_encoded_bytes(&mut self, _: &[u8]) -> Result<(), Self::Error> {
            Err(self.error)
        }
    }

    struct MockEngineConfig {
        padding: bool,
    }

    struct MockEngine {
        config: MockEngineConfig,
    }

    impl Config for MockEngineConfig {
        fn encode_padding(&self) -> bool {
            self.padding
        }
    }

    impl Engine for MockEngine {
        type Config = MockEngineConfig;
        type DecodeEstimate = ();

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            for (i, byte) in input.iter().enumerate() {
                if i < output.len() {
                    output[i] = *byte; // Mock encoding
                }
            }
            input.len()
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            ()
        }

        fn internal_decode(
            &self,
            _: &[u8],
            _: &mut [u8],
            _: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &self.config
        }
    }

    let mock_config = MockEngineConfig { padding: true };
    let mock_engine = MockEngine { config: mock_config };
    let encoder = ChunkedEncoder::new(&mock_engine);
    let mut sink = MockSink::new("Write Error");

    let input_data = vec![1, 2, 3, 4, 5];
    encoder.encode(&input_data, &mut sink).unwrap(); // This should panic
}


// Answer 0

#[test]
fn test_encode_with_full_chunk_and_sink_error() {
    struct MockSink {
        write_encoded_bytes_called: bool,
        write_error: Option<SinkError>,
    }
    
    impl Sink for MockSink {
        type Error = SinkError;

        fn write_encoded_bytes(&mut self, _bytes: &[u8]) -> Result<(), Self::Error> {
            self.write_encoded_bytes_called = true;
            // Simulate an error during write
            Err(SinkError)
        }
    }

    impl Config for MockConfig {
        fn encode_padding(&self) -> bool {
            true
        }
    }

    struct MockEngine;

    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            input.len() // Just for test sake
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            0 // Not needed for this test
        }

        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &MockConfig
        }
    }

    struct MockConfig;

    let engine = MockEngine;
    let encoder = ChunkedEncoder::new(&engine);
    let mut sink = MockSink {
        write_encoded_bytes_called: false,
        write_error: Some(SinkError),
    };
    
    let data = vec![0u8; 1024]; // data length at maximum
    let result = encoder.encode(&data, &mut sink);

    // The result should be an error due to sink write failure
}

#[test]
fn test_encode_with_partial_chunk_and_sink_error() {
    struct MockSink {
        write_encoded_bytes_called: bool,
        write_error: Option<SinkError>,
    }
    
    impl Sink for MockSink {
        type Error = SinkError;

        fn write_encoded_bytes(&mut self, _bytes: &[u8]) -> Result<(), Self::Error> {
            self.write_encoded_bytes_called = true;
            // Simulate an error during write
            Err(SinkError)
        }
    }

    impl Config for MockConfig {
        fn encode_padding(&self) -> bool {
            true
        }
    }

    struct MockEngine;

    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            input.len() // Just for test sake
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            0 // Not needed for this test
        }

        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &MockConfig
        }
    }

    struct MockConfig;

    let engine = MockEngine;
    let encoder = ChunkedEncoder::new(&engine);
    let mut sink = MockSink {
        write_encoded_bytes_called: false,
        write_error: Some(SinkError),
    };
    
    let data = vec![0u8; 700]; // less than CHUNK_SIZE
    let result = encoder.encode(&data, &mut sink);

    // The result should be an error due to sink write failure
}


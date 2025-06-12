// Answer 0

#[test]
fn test_encode_with_full_chunks() {
    struct DummyEngine;

    impl Engine for DummyEngine {
        type Config = DummyConfig;
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..input.len()].copy_from_slice(input);
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
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &DummyConfig
        }
    }

    struct DummyConfig;

    impl Config for DummyConfig {
        fn encode_padding(&self) -> bool {
            true
        }
    }

    struct DummySink {
        output: Vec<u8>,
    }

    impl DummySink {
        fn new() -> Self {
            DummySink { output: Vec::new() }
        }

        fn write_encoded_bytes(&mut self, bytes: &[u8]) -> Result<(), ()> {
            self.output.extend_from_slice(bytes);
            Ok(())
        }
    }

    let engine = DummyEngine;
    let encoder = ChunkedEncoder::new(&engine);
    let input_data = vec![1, 2, 3, 4, 5, 6]; // total 6 bytes, small enough to prevent chunk conditions
    let mut sink = DummySink::new();

    let result = encoder.encode(&input_data, &mut sink);
    assert!(result.is_ok());
    assert_eq!(sink.output, input_data);
}

#[test]
fn test_encode_with_partial_chunk_padding() {
    struct DummyEngine;

    impl Engine for DummyEngine {
        type Config = DummyConfig;
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            output[..input.len()].copy_from_slice(input);
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
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &DummyConfig
        }
    }

    struct DummyConfig;

    impl Config for DummyConfig {
        fn encode_padding(&self) -> bool {
            true
        }
    }

    struct DummySink {
        output: Vec<u8>,
    }

    impl DummySink {
        fn new() -> Self {
            DummySink { output: Vec::new() }
        }

        fn write_encoded_bytes(&mut self, bytes: &[u8]) -> Result<(), ()> {
            self.output.extend_from_slice(bytes);
            Ok(())
        }
    }

    let engine = DummyEngine;
    let encoder = ChunkedEncoder::new(&engine);
    let input_data = vec![1, 2, 3, 4, 5]; // 5 bytes to ensure padding is required
    let mut sink = DummySink::new();

    let result = encoder.encode(&input_data, &mut sink);
    assert!(result.is_ok());
    assert_eq!(sink.output, vec![1, 2, 3, 4, 5, 0, 0, 0]); // Expecting padding added
}


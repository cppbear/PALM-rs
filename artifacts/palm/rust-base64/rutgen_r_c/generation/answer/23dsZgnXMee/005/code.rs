// Answer 0

#[test]
fn test_encode_full_chunk_without_padding() {
    struct DummyEngine;
    struct DummyConfig;
    struct DummySink {
        encoded_data: Vec<u8>,
    }

    impl Engine for DummyEngine {
        type Config = DummyConfig;
        type DecodeEstimate = usize; // Placeholder for the type

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let len = input.len(); // Simulate encoding
            output[..len].copy_from_slice(input);
            len
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
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &DummyConfig
        }
    }

    impl Config for DummyConfig {
        fn encode_padding(&self) -> bool {
            false
        }
    }

    impl DummySink {
        fn new() -> Self {
            Self {
                encoded_data: Vec::new(),
            }
        }

        fn write_encoded_bytes(&mut self, data: &[u8]) -> Result<(), ()> {
            self.encoded_data.extend_from_slice(data);
            Ok(())
        }
    }

    let engine = DummyEngine;
    let encoder = ChunkedEncoder::new(&engine);
    let mut sink = DummySink::new();

    let input_data = vec![1u8; 1024]; // Full chunk, no padding
    let result = encoder.encode(&input_data, &mut sink);
    assert!(result.is_ok());
    assert_eq!(sink.encoded_data.len(), input_data.len());
}

#[test]
fn test_encode_partial_chunk_with_padding() {
    struct DummyEngine;
    struct DummyConfig;
    struct DummySink {
        encoded_data: Vec<u8>,
    }

    impl Engine for DummyEngine {
        type Config = DummyConfig;
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let len = input.len(); // Simulate encoding
            output[..len].copy_from_slice(input);
            len
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
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &DummyConfig
        }
    }

    impl Config for DummyConfig {
        fn encode_padding(&self) -> bool {
            true
        }
    }

    impl DummySink {
        fn new() -> Self {
            Self {
                encoded_data: Vec::new(),
            }
        }

        fn write_encoded_bytes(&mut self, data: &[u8]) -> Result<(), ()> {
            self.encoded_data.extend_from_slice(data);
            Ok(())
        }
    }

    let engine = DummyEngine;
    let encoder = ChunkedEncoder::new(&engine);
    let mut sink = DummySink::new();

    let input_data = vec![1u8; 1500]; // Partial chunk, padding needed
    let result = encoder.encode(&input_data, &mut sink);
    assert!(result.is_ok());
    assert!(!sink.encoded_data.is_empty());
}

#[test]
fn test_encode_sink_error() {
    struct DummyEngine;
    struct DummyConfig;

    struct DummySink {
        write_fail: bool,
    }

    impl Engine for DummyEngine {
        type Config = DummyConfig;
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            let len = input.len(); 
            output[..len].copy_from_slice(input);
            len
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
            unimplemented!()
        }

        fn config(&self) -> &Self::Config {
            &DummyConfig
        }
    }

    impl Config for DummyConfig {
        fn encode_padding(&self) -> bool {
            false
        }
    }

    impl DummySink {
        fn new(write_fail: bool) -> Self {
            Self { write_fail }
        }

        fn write_encoded_bytes(&mut self, _data: &[u8]) -> Result<(), ()> {
            if self.write_fail {
                Err(())
            } else {
                Ok(())
            }
        }
    }

    let engine = DummyEngine;
    let encoder = ChunkedEncoder::new(&engine);

    let mut sink = DummySink::new(true); // Set to fail write
    let input_data = vec![1u8; 1024]; // Full chunk
    let result = encoder.encode(&input_data, &mut sink);
    assert!(result.is_err());
}


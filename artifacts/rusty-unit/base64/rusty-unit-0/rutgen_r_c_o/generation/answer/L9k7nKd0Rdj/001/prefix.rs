// Answer 0

#[test]
fn test_chunked_encoder_new_valid_engine() {
    struct MockConfig;
    struct MockDecodeEstimate;
    struct MockEngine;

    impl Config for MockConfig {}
    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = MockDecodeEstimate;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            MockDecodeEstimate
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Err(DecodeSliceError::new())
        }

        fn config(&self) -> &Self::Config {
            &MockConfig
        }
    }

    let engine = MockEngine;
    let chunked_encoder = ChunkedEncoder::new(&engine);
}

#[test]
fn test_chunked_encoder_new_another_valid_engine() {
    struct AnotherMockConfig;
    struct AnotherMockDecodeEstimate;
    struct AnotherMockEngine;

    impl Config for AnotherMockConfig {}
    impl Engine for AnotherMockEngine {
        type Config = AnotherMockConfig;
        type DecodeEstimate = AnotherMockDecodeEstimate;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            AnotherMockDecodeEstimate
        }

        fn internal_decode(
            &self,
            _input: &[u8],
            _output: &mut [u8],
            _decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Err(DecodeSliceError::new())
        }

        fn config(&self) -> &Self::Config {
            &AnotherMockConfig
        }
    }

    let another_engine = AnotherMockEngine;
    let another_chunked_encoder = ChunkedEncoder::new(&another_engine);
}


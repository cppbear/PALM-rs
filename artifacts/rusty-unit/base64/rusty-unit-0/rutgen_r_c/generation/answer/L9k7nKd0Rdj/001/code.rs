// Answer 0

#[test]
fn test_chunked_encoder_new() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;
        
        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            0
        }
        
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }
        
        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata {})
        }
        
        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let mock_engine = MockEngine;
    let encoder = ChunkedEncoder::new(&mock_engine);
    assert_eq!(std::ptr::addr_of!(encoder.engine), std::ptr::addr_of!(mock_engine));
}

#[test]
fn test_chunked_encoder_new_with_different_lifetime() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata {})
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let mock_engine = MockEngine;
    {
        let encoder = ChunkedEncoder::new(&mock_engine);
        assert_eq!(std::ptr::addr_of!(encoder.engine), std::ptr::addr_of!(mock_engine));
    }
}


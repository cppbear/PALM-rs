// Answer 0

#[test]
fn test_config() {
    struct TestEngine {
        config: GeneralPurposeConfig,
    }

    impl Engine for TestEngine {
        type Config = GeneralPurposeConfig;
        type DecodeEstimate = GeneralPurposeEstimate;
        
        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, _: usize) -> Self::DecodeEstimate {
            GeneralPurposeEstimate { rem: 0, conservative_decoded_len: 0 }
        }

        fn internal_decode(
            &self,
            _: &[u8],
            _: &mut [u8],
            _: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { decoded_len: 0 })
        }

        fn config(&self) -> &Self::Config {
            &self.config
        }
    }

    let test_config = GeneralPurposeConfig {
        encode_padding: true,
        decode_allow_trailing_bits: false,
        decode_padding_mode: DecodePaddingMode::RequireNone,
    };

    let engine = TestEngine { config: test_config };
    
    assert_eq!(engine.config().encode_padding, true);
    assert_eq!(engine.config().decode_allow_trailing_bits, false);
    assert_eq!(engine.config().decode_padding_mode, DecodePaddingMode::RequireNone);
}


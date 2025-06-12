// Answer 0

#[test]
fn test_decode_to_buf_with_exact_length() {
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            // Dummy implementation
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            if input.is_empty() {
                return Err(DecodeSliceError::OutputSliceTooSmall);
            }
            // Simulate successful decoding
            let decoded_bytes = decode_estimate; // Just for testing purposes
            Ok(DecodeMetadata {
                decoded_len: decoded_bytes,
                padding_offset: None,
            })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = TestEngine;
    let input_data = b"SGVsbG8gd29ybGQ="; // "Hello world" in Base64
    let mut decoder_reader = DecoderReader::new(input_data.as_slice(), &engine);
    
    decoder_reader.b64_offset = 0;
    decoder_reader.b64_len = input_data.len();
    
    let mut output_buf = [0u8; 3]; // Buffer size based on estimate
    let result = decoder_reader.decode_to_buf(decoder_reader.b64_len, &mut output_buf);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 3); // We expect to decode 3 bytes
}

#[should_panic]
#[test]
fn test_decode_to_buf_panic_if_buf_is_empty() {
    struct TestEngine;

    impl Engine for TestEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, input: &[u8], output: &mut [u8]) -> usize {
            0
        }

        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate {
            input_len / 4 * 3
        }

        fn internal_decode(
            &self,
            input: &[u8],
            output: &mut [u8],
            decode_estimate: Self::DecodeEstimate,
        ) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata {
                decoded_len: decode_estimate,
                padding_offset: None,
            })
        }

        fn config(&self) -> &Self::Config {
            &()
        }
    }

    let engine = TestEngine;
    let input_data = b"SGVsbG8gd29ybGQ=";
    let mut decoder_reader = DecoderReader::new(input_data.as_slice(), &engine);
    
    decoder_reader.b64_offset = 0;
    decoder_reader.b64_len = input_data.len();
    
    let mut output_buf = []; // Empty buffer
    let _ = decoder_reader.decode_to_buf(decoder_reader.b64_len, &mut output_buf);
}


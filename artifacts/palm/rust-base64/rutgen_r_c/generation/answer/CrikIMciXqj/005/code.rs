// Answer 0

#[test]
fn test_flush_decoded_buf_with_valid_input() {
    struct MockEngine;
    struct MockConfig;
    struct MockDecodeEstimate;

    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = MockDecodeEstimate;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }
        
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            MockDecodeEstimate
        }
        
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { decoded_len: 0 })
        }

        fn config(&self) -> &Self::Config {
            &MockConfig
        }
    }

    let engine = MockEngine;
    let mut decoder = DecoderReader::new(std::io::Cursor::new(&b"test base64 data"[..]), &engine);
    
    decoder.decoded_chunk_buffer[..3].copy_from_slice(&[1, 2, 3]);
    decoder.decoded_len = 3;
    decoder.decoded_offset = 0;

    let mut output_buffer = [0u8; 10];
    let result = decoder.flush_decoded_buf(&mut output_buffer);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 3);
    assert_eq!(&output_buffer[..3], &[1, 2, 3]);
    assert_eq!(decoder.decoded_len, 0);
    assert_eq!(decoder.decoded_offset, 3);
}

#[test]
#[should_panic]
fn test_flush_decoded_buf_with_empty_buffer() {
    struct MockEngine;
    struct MockConfig;
    struct MockDecodeEstimate;

    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = MockDecodeEstimate;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }
        
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            MockDecodeEstimate
        }
        
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { decoded_len: 0 })
        }

        fn config(&self) -> &Self::Config {
            &MockConfig
        }
    }

    let engine = MockEngine;
    let mut decoder = DecoderReader::new(std::io::Cursor::new(&b"test base64 data"[..]), &engine);
    
    decoder.decoded_chunk_buffer[..3].copy_from_slice(&[1, 2, 3]);
    decoder.decoded_len = 3;
    decoder.decoded_offset = 0;

    let mut output_buffer = [];
    
    // This should panic due to the output buffer being empty
    let _ = decoder.flush_decoded_buf(&mut output_buffer);
}

#[test]
#[should_panic]
fn test_flush_decoded_buf_with_zero_decoded_len() {
    struct MockEngine;
    struct MockConfig;
    struct MockDecodeEstimate;

    impl Engine for MockEngine {
        type Config = MockConfig;
        type DecodeEstimate = MockDecodeEstimate;

        fn internal_encode(&self, _input: &[u8], _output: &mut [u8]) -> usize {
            0
        }
        
        fn internal_decoded_len_estimate(&self, _input_len: usize) -> Self::DecodeEstimate {
            MockDecodeEstimate
        }
        
        fn internal_decode(&self, _input: &[u8], _output: &mut [u8], _decode_estimate: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { decoded_len: 0 })
        }

        fn config(&self) -> &Self::Config {
            &MockConfig
        }
    }

    let engine = MockEngine;
    let mut decoder = DecoderReader::new(std::io::Cursor::new(&b"test base64 data"[..]), &engine);
    
    decoder.decoded_len = 0; // Set decoded_len to 0
    decoder.decoded_offset = 0;

    let mut output_buffer = [0u8; 10];
    
    // This should panic due to decoded_len being 0
    let _ = decoder.flush_decoded_buf(&mut output_buffer);
}


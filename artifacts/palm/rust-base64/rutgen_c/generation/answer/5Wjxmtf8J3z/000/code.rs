// Answer 0

#[test]
fn test_decode_to_buf_valid_input() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate { input_len * 3 / 4 }
        fn internal_decode(&self, input: &[u8], output: &mut [u8], _: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            output.copy_from_slice(&input[0..input.len().min(output.len())]);
            Ok(DecodeMetadata { decoded_len: input.len().min(output.len()), padding_offset: None })
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let mut buffer = [0u8; 4];
    let mut decoder = DecoderReader::new(&b"SGVsbG8="[..], &MockEngine);
    let result = decoder.decode_to_buf(8, &mut buffer);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 4);
    assert_eq!(&buffer[..4], b"Hello");
}

#[test]
#[should_panic]
fn test_decode_to_buf_panic_on_small_buffer() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate { input_len * 3 / 4 }
        fn internal_decode(&self, _: &[u8], output: &mut [u8], _: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            Ok(DecodeMetadata { decoded_len: 0, padding_offset: None })
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let mut decoder = DecoderReader::new(&b"SGVsbG8="[..], &MockEngine);
    let _ = decoder.decode_to_buf(8, &mut []); // This should panic
}

#[test]
fn test_decode_to_buf_insufficient_data() {
    struct MockEngine;
    impl Engine for MockEngine {
        type Config = ();
        type DecodeEstimate = usize;

        fn internal_encode(&self, _: &[u8], _: &mut [u8]) -> usize { 0 }
        fn internal_decoded_len_estimate(&self, input_len: usize) -> Self::DecodeEstimate { input_len * 3 / 4 }
        fn internal_decode(&self, input: &[u8], output: &mut [u8], _: Self::DecodeEstimate) -> Result<DecodeMetadata, DecodeSliceError> {
            if input.is_empty() {
                return Err(DecodeSliceError::OutputSliceTooSmall);
            }
            output.copy_from_slice(&input[0..output.len()]);
            Ok(DecodeMetadata { decoded_len: output.len(), padding_offset: None })
        }
        fn config(&self) -> &Self::Config { &() }
    }

    let mut buffer = [0u8; 2];
    let mut decoder = DecoderReader::new(&b""[..], &MockEngine);
    let result = decoder.decode_to_buf(0, &mut buffer);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}


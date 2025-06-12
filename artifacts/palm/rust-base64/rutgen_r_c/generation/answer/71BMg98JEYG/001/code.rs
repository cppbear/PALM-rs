// Answer 0

#[test]
fn test_encode_engine_slice_success() {
    struct MockEngine;

    impl Engine for MockEngine {
        fn encode_slice<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut [u8]) -> Result<usize, EncodeSliceError> {
            let input_slice = input.as_ref();
            let output = base64::encode(input_slice);
            let bytes = output.as_bytes();
            if bytes.len() > output_buf.len() {
                return Err(EncodeSliceError::OutputSliceTooSmall);
            }
            output_buf[..bytes.len()].copy_from_slice(bytes);
            Ok(bytes.len())
        }
    }

    let engine = MockEngine;
    let input = b"hello";
    let mut output_buf = vec![0u8; 8]; // Sufficient size for base64 encoding of "hello"
    let result = encode_engine_slice(input, &mut output_buf, &engine);

    assert_eq!(result, Ok(8)); // Base64 of "hello" is "aGVsbG8=" which is 8 bytes
    assert_eq!(&output_buf[..8], b"aGVsbG8="); // Validate the output buffer content
}

#[test]
fn test_encode_engine_slice_output_buffer_too_small() {
    struct MockEngine;

    impl Engine for MockEngine {
        fn encode_slice<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut [u8]) -> Result<usize, EncodeSliceError> {
            let input_slice = input.as_ref();
            let output = base64::encode(input_slice);
            let bytes = output.as_bytes();
            if bytes.len() > output_buf.len() {
                return Err(EncodeSliceError::OutputSliceTooSmall);
            }
            output_buf[..bytes.len()].copy_from_slice(bytes);
            Ok(bytes.len())
        }
    }

    let engine = MockEngine;
    let input = b"world";
    let mut output_buf = vec![0u8; 5]; // Inadequate size for base64 encoding of "world"
    let result = encode_engine_slice(input, &mut output_buf, &engine);

    assert_eq!(result, Err(EncodeSliceError::OutputSliceTooSmall));
}


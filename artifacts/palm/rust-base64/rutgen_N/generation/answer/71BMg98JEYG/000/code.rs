// Answer 0

#[test]
fn test_encode_engine_slice_valid_case() {
    struct MockEngine;

    impl Engine for MockEngine {
        fn encode_slice<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut [u8]) -> Result<usize, EncodeSliceError> {
            let input_bytes = input.as_ref();
            let encoded = base64::encode(input_bytes);
            let output = encoded.as_bytes();
            if output_buf.len() < output.len() {
                return Err(EncodeSliceError::BufferTooSmall);
            }
            output_buf[..output.len()].copy_from_slice(output);
            Ok(output.len())
        }
    }

    let engine = MockEngine;
    let input = b"Hello, world!";
    let mut output_buffer = vec![0u8; 20];
    let result = encode_engine_slice(&input[..], &mut output_buffer, &engine);
    
    assert_eq!(result, Ok(16));
    assert_eq!(&output_buffer[..16], b"SGVsbG8sIHdvcmxkIQ==");
}

#[test]
fn test_encode_engine_slice_buffer_too_small() {
    struct MockEngine;

    impl Engine for MockEngine {
        fn encode_slice<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut [u8]) -> Result<usize, EncodeSliceError> {
            let input_bytes = input.as_ref();
            let encoded = base64::encode(input_bytes);
            let output = encoded.as_bytes();
            if output_buf.len() < output.len() {
                return Err(EncodeSliceError::BufferTooSmall);
            }
            output_buf[..output.len()].copy_from_slice(output);
            Ok(output.len())
        }
    }

    let engine = MockEngine;
    let input = b"Hello!";
    let mut output_buffer = vec![0u8; 5]; // Smaller than needed
    let result = encode_engine_slice(&input[..], &mut output_buffer, &engine);
    
    assert_eq!(result, Err(EncodeSliceError::BufferTooSmall));
}

#[test]
fn test_encode_engine_slice_empty_input() {
    struct MockEngine;

    impl Engine for MockEngine {
        fn encode_slice<T: AsRef<[u8]>>(&self, input: T, output_buf: &mut [u8]) -> Result<usize, EncodeSliceError> {
            let input_bytes = input.as_ref();
            let encoded = base64::encode(input_bytes);
            let output = encoded.as_bytes();
            if output_buf.len() < output.len() {
                return Err(EncodeSliceError::BufferTooSmall);
            }
            output_buf[..output.len()].copy_from_slice(output);
            Ok(output.len())
        }
    }

    let engine = MockEngine;
    let input: &[u8] = &[];
    let mut output_buffer = vec![0u8; 20];
    let result = encode_engine_slice(input, &mut output_buffer, &engine);
    
    assert_eq!(result, Ok(0));
    assert_eq!(&output_buffer[..0], b""); // No output for empty input
}


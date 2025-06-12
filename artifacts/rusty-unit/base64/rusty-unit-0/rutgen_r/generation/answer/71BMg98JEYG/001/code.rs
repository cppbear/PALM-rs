// Answer 0

#[test]
fn test_encode_engine_slice_success() {
    struct MockEngine;

    impl Engine for MockEngine {
        fn encode_slice<T: AsRef<[u8]>>(
            &self,
            input: T,
            output_buf: &mut [u8],
        ) -> Result<usize, EncodeSliceError> {
            let input_data = input.as_ref();
            let output_len = base64::encode(&input_data).len();
            if output_buf.len() < output_len {
                return Err(EncodeSliceError::BufferTooSmall);
            }
            let encoded = base64::encode(&input_data);
            output_buf[..output_len].copy_from_slice(encoded.as_bytes());
            Ok(output_len)
        }
    }

    let input_data = b"Hello, world!";
    let mut output_buf = vec![0u8; 100]; // Large enough buffer
    let engine = MockEngine;

    let result = encode_engine_slice(input_data, &mut output_buf, &engine);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), base64::encode(input_data).len());
}

#[test]
#[should_panic]
fn test_encode_engine_slice_buffer_too_small() {
    struct MockEngine;

    impl Engine for MockEngine {
        fn encode_slice<T: AsRef<[u8]>>(
            &self,
            input: T,
            output_buf: &mut [u8],
        ) -> Result<usize, EncodeSliceError> {
            let input_data = input.as_ref();
            let output_len = base64::encode(&input_data).len();
            if output_buf.len() < output_len {
                return Err(EncodeSliceError::BufferTooSmall);
            }
            let encoded = base64::encode(&input_data);
            output_buf[..output_len].copy_from_slice(encoded.as_bytes());
            Ok(output_len)
        }
    }

    let input_data = b"Hello, world!";
    let mut output_buf = vec![0u8; 5]; // Intentionally too small buffer
    let engine = MockEngine;

    encode_engine_slice(input_data, &mut output_buf, &engine).unwrap();
}

#[test]
fn test_encode_engine_slice_empty_input() {
    struct MockEngine;

    impl Engine for MockEngine {
        fn encode_slice<T: AsRef<[u8]>>(
            &self,
            input: T,
            output_buf: &mut [u8],
        ) -> Result<usize, EncodeSliceError> {
            let input_data = input.as_ref();
            let output_len = base64::encode(&input_data).len();
            if output_buf.len() < output_len {
                return Err(EncodeSliceError::BufferTooSmall);
            }
            let encoded = base64::encode(&input_data);
            output_buf[..output_len].copy_from_slice(encoded.as_bytes());
            Ok(output_len)
        }
    }

    let input_data = b"";
    let mut output_buf = vec![0u8; 100]; // Large enough buffer
    let engine = MockEngine;

    let result = encode_engine_slice(input_data, &mut output_buf, &engine);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_encode_engine_slice_large_input() {
    struct MockEngine;

    impl Engine for MockEngine {
        fn encode_slice<T: AsRef<[u8]>>(
            &self,
            input: T,
            output_buf: &mut [u8],
        ) -> Result<usize, EncodeSliceError> {
            let input_data = input.as_ref();
            let output_len = base64::encode(&input_data).len();
            if output_buf.len() < output_len {
                return Err(EncodeSliceError::BufferTooSmall);
            }
            let encoded = base64::encode(&input_data);
            output_buf[..output_len].copy_from_slice(encoded.as_bytes());
            Ok(output_len)
        }
    }

    let input_data = b"The quick brown fox jumps over the lazy dog";
    let mut output_buf = vec![0u8; 100]; // Adequate buffer
    let engine = MockEngine;

    let result = encode_engine_slice(input_data, &mut output_buf, &engine);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), base64::encode(input_data).len());
}


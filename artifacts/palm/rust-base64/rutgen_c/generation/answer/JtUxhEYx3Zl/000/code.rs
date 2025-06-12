// Answer 0

#[test]
fn test_decode_engine_slice_success() {
    struct TestEngine;

    impl Engine for TestEngine {
        fn decode_slice<T: AsRef<[u8]>>(&self, input: T, output: &mut [u8]) -> Result<usize, DecodeSliceError> {
            let input_bytes = input.as_ref();
            if input_bytes == b"SGVsbG8=" { // "Hello" in base64
                if output.len() < 5 {
                    return Err(DecodeSliceError::OutputSliceTooSmall);
                }
                output[..5].copy_from_slice(b"Hello");
                return Ok(5);
            }
            Err(DecodeSliceError::DecodeError(DecodeError))
        }
    }

    let engine = TestEngine;
    let mut output = [0u8; 10];
    let result = decode_engine_slice(b"SGVsbG8=", &mut output[..], &engine);
    assert_eq!(result, Ok(5));
    assert_eq!(&output[..5], b"Hello");
}

#[test]
fn test_decode_engine_slice_output_too_small() {
    struct TestEngine;

    impl Engine for TestEngine {
        fn decode_slice<T: AsRef<[u8]>>(&self, input: T, output: &mut [u8]) -> Result<usize, DecodeSliceError> {
            let input_bytes = input.as_ref();
            if input_bytes == b"SGVsbG8=" {
                return Err(DecodeSliceError::OutputSliceTooSmall);
            }
            Err(DecodeSliceError::DecodeError(DecodeError))
        }
    }

    let engine = TestEngine;
    let mut output = [0u8; 4]; // Not enough space for "Hello"
    let result = decode_engine_slice(b"SGVsbG8=", &mut output[..], &engine);
    assert_eq!(result, Err(DecodeSliceError::OutputSliceTooSmall));
}

#[test]
fn test_decode_engine_slice_invalid_input() {
    struct TestEngine;

    impl Engine for TestEngine {
        fn decode_slice<T: AsRef<[u8]>>(&self, input: T, output: &mut [u8]) -> Result<usize, DecodeSliceError> {
            Err(DecodeSliceError::DecodeError(DecodeError))
        }
    }

    let engine = TestEngine;
    let mut output = [0u8; 10];
    let result = decode_engine_slice(b"InvalidBase64", &mut output[..], &engine);
    assert_eq!(result, Err(DecodeSliceError::DecodeError(DecodeError)));
}


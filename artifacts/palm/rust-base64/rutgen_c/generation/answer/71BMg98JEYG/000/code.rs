// Answer 0

#[derive(Clone, Debug, PartialEq, Eq)]
struct MockEngine;

impl Engine for MockEngine {
    fn encode_slice<T: AsRef<[u8]>>(&self, _: T, output_buf: &mut [u8]) -> Result<usize, EncodeSliceError> {
        if output_buf.len() < 4 {
            return Err(EncodeSliceError::OutputSliceTooSmall);
        }
        // Mock encoding; for testing, we'll just write fake data
        output_buf[..4].copy_from_slice(b"Test");
        Ok(4)
    }
}

#[test]
fn test_encode_engine_slice_success() {
    let input = b"test";
    let mut output_buf = [0u8; 4];
    let engine = MockEngine;

    let result = encode_engine_slice(input, &mut output_buf, &engine);
    
    assert_eq!(result, Ok(4));
    assert_eq!(&output_buf, b"Test");
}

#[test]
fn test_encode_engine_slice_output_slice_too_small() {
    let input = b"test";
    let mut output_buf = [0u8; 3]; // Size is too small
    let engine = MockEngine;

    let result = encode_engine_slice(input, &mut output_buf, &engine);
    
    assert_eq!(result, Err(EncodeSliceError::OutputSliceTooSmall));
}


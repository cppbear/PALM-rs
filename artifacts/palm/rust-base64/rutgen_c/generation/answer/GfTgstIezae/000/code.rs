// Answer 0

#[test]
fn test_fmt_decode_error() {
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct MockDecodeError(usize, u8);

    impl fmt::Display for MockDecodeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "MockDecodeError: invalid byte {} at offset {}", self.1, self.0)
        }
    }

    let decode_error = DecodeSliceError::DecodeError(DecodeError::InvalidByte(5, b'x'));
    let mut output = String::new();
    let result = write!(&mut output, "{}", decode_error);
    assert!(result.is_ok());
    assert_eq!(output, "DecodeError: MockDecodeError: invalid byte 120 at offset 5");
}

#[test]
fn test_fmt_output_slice_too_small() {
    let output_slice_too_small = DecodeSliceError::OutputSliceTooSmall;
    let mut output = String::new();
    let result = write!(&mut output, "{}", output_slice_too_small);
    assert!(result.is_ok());
    assert_eq!(output, "Output slice too small");
}


// Answer 0

#[test]
fn test_source_with_decode_error() {
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct TestDecodeError {
        offset: usize,
        byte: u8,
    }

    impl fmt::Debug for TestDecodeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "InvalidByte({}, {})", self.offset, self.byte)
        }
    }

    let decode_error = DecodeError::InvalidByte(3, b'x');
    let error_variant = DecodeSliceError::DecodeError(decode_error.clone());

    assert_eq!(error_variant.source(), Some(&decode_error));
}

#[test]
fn test_source_with_output_slice_too_small() {
    let error_variant = DecodeSliceError::OutputSliceTooSmall;

    assert_eq!(error_variant.source(), None);
}


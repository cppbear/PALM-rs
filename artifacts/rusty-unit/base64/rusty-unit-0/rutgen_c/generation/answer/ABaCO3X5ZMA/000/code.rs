// Answer 0

#[test]
fn test_source_with_decode_error() {
    #[derive(Clone, Debug, PartialEq, Eq)]
    struct TestDecodeError(usize, u8);

    impl fmt::Debug for TestDecodeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "TestDecodeError({}, {})", self.0, self.1)
        }
    }

    let error = DecodeSliceError::DecodeError(TestDecodeError(5, b'A'));
    assert_eq!(error.source().is_some(), true);
}

#[test]
fn test_source_with_output_slice_too_small() {
    let error = DecodeSliceError::OutputSliceTooSmall;
    assert_eq!(error.source(), None);
}


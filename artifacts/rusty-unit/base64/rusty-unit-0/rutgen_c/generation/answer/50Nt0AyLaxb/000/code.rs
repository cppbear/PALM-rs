// Answer 0

#[test]
fn test_encode_slice_error_display() {
    let error = EncodeSliceError::OutputSliceTooSmall;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error);
    assert!(result.is_ok());
    assert_eq!(buffer, "Output slice too small");
}


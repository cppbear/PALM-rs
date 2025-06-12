// Answer 0

#[test]
fn test_encode_slice_error_display() {
    let error = EncodeSliceError::OutputSliceTooSmall;
    let result = format!("{}", error);
    assert_eq!(result, "Output slice too small");
}


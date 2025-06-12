// Answer 0

#[test]
fn test_encode_slice_error_display_output_slice_too_small() {
    let error = EncodeSliceError::OutputSliceTooSmall;
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", error);
}

#[test]
fn test_encode_slice_error_display_output_slice_too_small_alternative() {
    let error = EncodeSliceError::OutputSliceTooSmall;
    let mut buffer = String::new();
    let _ = error.fmt(&mut buffer);
}

#[test]
#[should_panic]
fn test_encode_slice_error_panic_on_large_string() {
    let error = EncodeSliceError::OutputSliceTooSmall;
    let mut buffer = String::with_capacity(1001);
    let _ = write!(&mut buffer, "{}", error);
}


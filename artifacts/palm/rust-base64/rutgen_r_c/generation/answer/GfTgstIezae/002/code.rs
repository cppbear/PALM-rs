// Answer 0

#[test]
fn test_fmt_decode_error_invalid_byte() {
    let error = super::DecodeError::InvalidByte(3, b'a');
    let decode_error = super::DecodeSliceError::DecodeError(error);

    let mut output = String::new();
    let result = write!(&mut output, "{}", decode_error);
    assert!(result.is_ok());
    assert_eq!(output, "DecodeError: InvalidByte(3, 97)");
}

#[test]
fn test_fmt_decode_error_invalid_length() {
    let error = super::DecodeError::InvalidLength(5);
    let decode_error = super::DecodeSliceError::DecodeError(error);

    let mut output = String::new();
    let result = write!(&mut output, "{}", decode_error);
    assert!(result.is_ok());
    assert_eq!(output, "DecodeError: InvalidLength(5)");
}

#[test]
fn test_fmt_decode_error_invalid_last_symbol() {
    let error = super::DecodeError::InvalidLastSymbol(4, b'z');
    let decode_error = super::DecodeSliceError::DecodeError(error);

    let mut output = String::new();
    let result = write!(&mut output, "{}", decode_error);
    assert!(result.is_ok());
    assert_eq!(output, "DecodeError: InvalidLastSymbol(4, 122)");
}

#[test]
fn test_fmt_decode_error_invalid_padding() {
    let error = super::DecodeError::InvalidPadding;
    let decode_error = super::DecodeSliceError::DecodeError(error);

    let mut output = String::new();
    let result = write!(&mut output, "{}", decode_error);
    assert!(result.is_ok());
    assert_eq!(output, "DecodeError: InvalidPadding");
}

#[test]
fn test_fmt_output_slice_too_small() {
    let decode_error = super::DecodeSliceError::OutputSliceTooSmall;

    let mut output = String::new();
    let result = write!(&mut output, "{}", decode_error);
    assert!(result.is_ok());
    assert_eq!(output, "Output slice too small");
}


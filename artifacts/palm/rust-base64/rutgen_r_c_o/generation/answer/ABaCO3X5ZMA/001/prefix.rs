// Answer 0

#[test]
fn test_source_output_slice_too_small() {
    let error = DecodeSliceError::OutputSliceTooSmall;
    let result = error.source();
}

#[test]
fn test_source_decode_error() {
    let error = DecodeSliceError::DecodeError(DecodeError::InvalidByte(5, b'x'));
    let result = error.source();
}

#[test]
fn test_source_invalid_length() {
    let error = DecodeSliceError::DecodeError(DecodeError::InvalidLength(2));
    let result = error.source();
}

#[test]
fn test_source_invalid_last_symbol() {
    let error = DecodeSliceError::DecodeError(DecodeError::InvalidLastSymbol(1, b'y'));
    let result = error.source();
}

#[test]
fn test_source_invalid_padding() {
    let error = DecodeSliceError::DecodeError(DecodeError::InvalidPadding);
    let result = error.source();
}


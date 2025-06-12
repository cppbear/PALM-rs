// Answer 0


use std::fmt;

enum CustomError {
    DecodeError(String),
    OutputSliceTooSmall,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DecodeError(e) => write!(f, "DecodeError: {}", e),
            Self::OutputSliceTooSmall => write!(f, "Output slice too small"),
        }
    }
}

#[test]
fn test_fmt_decode_error() {
    let error_message = "Invalid Base64 string".to_string();
    let error_instance = CustomError::DecodeError(error_message);
    let result = format!("{}", error_instance);
    assert_eq!(result, "DecodeError: Invalid Base64 string");
}

#[test]
fn test_fmt_output_slice_too_small() {
    let error_instance = CustomError::OutputSliceTooSmall;
    let result = format!("{}", error_instance);
    assert_eq!(result, "Output slice too small");
}



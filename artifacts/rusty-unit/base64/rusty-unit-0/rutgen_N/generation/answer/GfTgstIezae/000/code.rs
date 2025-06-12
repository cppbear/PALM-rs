// Answer 0

#[derive(Debug)]
enum DecodeError {
    DecodeError(String),
    OutputSliceTooSmall,
}

impl std::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DecodeError(e) => write!(f, "DecodeError: {}", e),
            Self::OutputSliceTooSmall => write!(f, "Output slice too small"),
        }
    }
}

#[test]
fn test_fmt_decode_error() {
    let error = DecodeError::DecodeError("Invalid character".to_string());
    let result = format!("{}", error);
    assert_eq!(result, "DecodeError: Invalid character");
}

#[test]
fn test_fmt_output_slice_too_small() {
    let error = DecodeError::OutputSliceTooSmall;
    let result = format!("{}", error);
    assert_eq!(result, "Output slice too small");
}


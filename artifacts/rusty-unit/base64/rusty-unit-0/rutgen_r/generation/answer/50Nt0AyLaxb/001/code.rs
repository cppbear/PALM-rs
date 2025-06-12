// Answer 0


use std::fmt;

enum Error {
    OutputSliceTooSmall,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutputSliceTooSmall => write!(f, "Output slice too small"),
        }
    }
}

#[test]
fn test_output_slice_too_small_debug() {
    let error = Error::OutputSliceTooSmall;
    let result = format!("{:?}", error);
    assert_eq!(result, "Output slice too small");
}

#[test]
fn test_output_slice_too_small_display() {
    let error = Error::OutputSliceTooSmall;
    let result = format!("{}", error);
    assert_eq!(result, "Output slice too small");
}



// Answer 0

#[derive(Debug)]
enum StrSimError {
    DifferentLengthArgs,
}

impl std::fmt::Display for StrSimError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let text = match self {
            StrSimError::DifferentLengthArgs => "Differing length arguments provided",
        };
        write!(fmt, "{}", text)
    }
}

#[test]
fn test_fmt_different_length_args() {
    let error = StrSimError::DifferentLengthArgs;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error);

    assert!(result.is_ok());
    assert_eq!(buffer, "Differing length arguments provided");
}

#[test]
fn test_fmt_empty_string() {
    let error = StrSimError::DifferentLengthArgs;
    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error);

    assert!(result.is_ok());
    assert_eq!(buffer, "Differing length arguments provided");
}


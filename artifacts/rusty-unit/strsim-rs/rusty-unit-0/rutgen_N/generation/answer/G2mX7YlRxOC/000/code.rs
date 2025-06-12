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
    let mut output = String::new();
    let result = error.fmt(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "Differing length arguments provided");
}


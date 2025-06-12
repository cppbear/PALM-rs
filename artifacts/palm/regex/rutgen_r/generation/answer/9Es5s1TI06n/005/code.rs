// Answer 0

#[derive(Debug)]
enum ErrorKind {
    RepetitionCountUnclosed,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ErrorKind::*;
        match *self {
            RepetitionCountUnclosed => write!(f, "unclosed counted repetition"),
        }
    }
}

#[test]
fn test_repetition_count_unclosed() {
    let error = ErrorKind::RepetitionCountUnclosed;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "unclosed counted repetition");
}


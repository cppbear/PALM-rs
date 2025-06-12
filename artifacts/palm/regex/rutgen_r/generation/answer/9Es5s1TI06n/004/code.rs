// Answer 0

#[test]
fn test_fmt_repetition_missing() {
    use std::fmt;

    #[derive(Debug)]
    enum ErrorKind {
        RepetitionMissing,
    }

    impl fmt::Display for ErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                ErrorKind::RepetitionMissing => {
                    write!(f, "repetition operator missing expression")
                }
            }
        }
    }

    let error = ErrorKind::RepetitionMissing;
    let result = format!("{}", error);
    assert_eq!(result, "repetition operator missing expression");
}

#[test]
fn test_fmt_repetition_missing_multiple_times() {
    use std::fmt;

    #[derive(Debug)]
    enum ErrorKind {
        RepetitionMissing,
    }

    impl fmt::Display for ErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
                ErrorKind::RepetitionMissing => {
                    write!(f, "repetition operator missing expression")
                }
            }
        }
    }

    let error = ErrorKind::RepetitionMissing;
    let result1 = format!("{}", error);
    let result2 = format!("{}", error);
    assert_eq!(result1, "repetition operator missing expression");
    assert_eq!(result2, "repetition operator missing expression");
}


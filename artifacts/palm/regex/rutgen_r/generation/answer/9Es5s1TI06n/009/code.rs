// Answer 0

#[test]
fn test_group_unclosed_fmt() {
    use std::fmt;

    struct Error {
        kind: ErrorKind,
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.kind {
                ErrorKind::GroupUnclosed => write!(f, "unclosed group"),
                _ => unreachable!(),
            }
        }
    }

    enum ErrorKind {
        GroupUnclosed,
    }

    let error = Error { kind: ErrorKind::GroupUnclosed };
    let mut output = String::new();
    let result = error.fmt(&mut fmt::Formatter::new(&mut output));

    assert!(result.is_ok());
    assert_eq!(output, "unclosed group");
}

#[test]
fn test_group_name_empty_fmt() {
    use std::fmt;

    struct Error {
        kind: ErrorKind,
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.kind {
                ErrorKind::GroupNameEmpty => write!(f, "empty capture group name"),
                _ => unreachable!(),
            }
        }
    }

    enum ErrorKind {
        GroupNameEmpty,
    }

    let error = Error { kind: ErrorKind::GroupNameEmpty };
    let mut output = String::new();
    let result = error.fmt(&mut fmt::Formatter::new(&mut output));

    assert!(result.is_ok());
    assert_eq!(output, "empty capture group name");
}


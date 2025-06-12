// Answer 0

#[derive(Debug)]
struct ErrorKind;

impl ErrorKind {
    const FlagUnrecognized: ErrorKind = ErrorKind;

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::ErrorKind::*;
        match *self {
            FlagUnrecognized => {
                write!(f, "unrecognized flag")
            }
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_flag_unrecognized_fmt() {
    let error = ErrorKind::FlagUnrecognized;
    let mut output = String::new();
    let result = error.fmt(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "unrecognized flag");
}

#[derive(Debug)]
struct AnotherErrorKind;

impl AnotherErrorKind {
    const FlagUnrecognized: AnotherErrorKind = AnotherErrorKind;

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::AnotherErrorKind::*;
        match *self {
            FlagUnrecognized => {
                write!(f, "unrecognized flag")
            }
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_another_flag_unrecognized_fmt() {
    let error = AnotherErrorKind::FlagUnrecognized;
    let mut output = String::new();
    let result = error.fmt(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "unrecognized flag");
}


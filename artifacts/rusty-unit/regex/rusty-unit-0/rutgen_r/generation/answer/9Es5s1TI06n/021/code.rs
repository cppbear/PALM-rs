// Answer 0

#[derive(Debug)]
enum ErrorKind {
    EscapeHexInvalidDigit,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::ErrorKind::*;
        match *self {
            EscapeHexInvalidDigit => {
                write!(f, "invalid hexadecimal digit")
            }
        }
    }
}

#[test]
fn test_escape_hex_invalid_digit() {
    let error = ErrorKind::EscapeHexInvalidDigit;
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "invalid hexadecimal digit");
}

#[test]
#[should_panic]
fn test_unreachable() {
    #[derive(Debug)]
    enum Unreachable {}
    
    impl std::fmt::Display for Unreachable {
        fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
            unreachable!()
        }
    }

    let error = Unreachable {};
    let _ = format!("{}", error);
}


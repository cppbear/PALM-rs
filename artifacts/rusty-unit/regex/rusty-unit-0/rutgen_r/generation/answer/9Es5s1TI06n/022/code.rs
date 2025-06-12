// Answer 0

#[test]
fn test_escape_hex_invalid() {
    use std::fmt;
    
    struct ErrorKind {
        kind: String,
    }

    impl fmt::Display for ErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.kind.as_str() {
                "EscapeHexInvalid" => write!(f, "hexadecimal literal is not a Unicode scalar value"),
                _ => unreachable!(),
            }
        }
    }

    let error = ErrorKind {
        kind: "EscapeHexInvalid".to_string(),
    };

    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "hexadecimal literal is not a Unicode scalar value");
}

#[test]
fn test_escape_hex_empty() {
    use std::fmt;
    
    struct ErrorKind {
        kind: String,
    }

    impl fmt::Display for ErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.kind.as_str() {
                "EscapeHexEmpty" => write!(f, "hexadecimal literal empty"),
                _ => unreachable!(),
            }
        }
    }

    let error = ErrorKind {
        kind: "EscapeHexEmpty".to_string(),
    };

    let mut buffer = String::new();
    let result = write!(&mut buffer, "{}", error);
    
    assert!(result.is_ok());
    assert_eq!(buffer, "hexadecimal literal empty");
}


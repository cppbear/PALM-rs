// Answer 0

#[test]
fn test_fmt_group_name_empty() {
    struct ErrorKind {
        kind: String,
    }

    impl fmt::Display for ErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.kind.as_str() {
                "GroupNameEmpty" => write!(f, "empty capture group name"),
                _ => write!(f, "unknown error"),
            }
        }
    }

    let error = ErrorKind { kind: "GroupNameEmpty".to_string() };
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    
    assert!(result.is_ok());
    assert_eq!(output, "empty capture group name");
}

#[test]
fn test_fmt_group_name_duplicate() {
    struct ErrorKind {
        kind: String,
    }

    impl fmt::Display for ErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.kind.as_str() {
                "GroupNameDuplicate" => write!(f, "duplicate capture group name"),
                _ => write!(f, "unknown error"),
            }
        }
    }

    let error = ErrorKind { kind: "GroupNameDuplicate".to_string() };
    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    
    assert!(result.is_ok());
    assert_eq!(output, "duplicate capture group name");
}


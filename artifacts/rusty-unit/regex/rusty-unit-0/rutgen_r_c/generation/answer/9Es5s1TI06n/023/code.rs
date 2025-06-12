// Answer 0

#[test]
fn test_fmt_escape_hex_empty() {
    struct TestErrorKind {
        kind: ErrorKind,
    }

    impl fmt::Display for TestErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt(&self.kind, f)
        }
    }

    let error = TestErrorKind {
        kind: ErrorKind::EscapeHexEmpty,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "hexadecimal literal empty");
}

#[test]
fn test_fmt_class_escape_invalid() {
    struct TestErrorKind {
        kind: ErrorKind,
    }

    impl fmt::Display for TestErrorKind {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt(&self.kind, f)
        }
    }

    let error = TestErrorKind {
        kind: ErrorKind::ClassEscapeInvalid,
    };

    let mut output = String::new();
    let result = write!(&mut output, "{}", error);
    assert!(result.is_ok());
    assert_eq!(output, "invalid escape sequence found in character class");
}


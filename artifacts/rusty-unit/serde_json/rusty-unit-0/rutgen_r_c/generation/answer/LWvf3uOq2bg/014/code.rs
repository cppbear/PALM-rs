// Answer 0

#[test]
fn test_error_code_display_expected_double_quote() {
    use core::fmt::Formatter;

    struct MockFormatter {
        output: String,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter {
                output: String::new(),
            }
        }
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut formatter = MockFormatter::new();
    let error_code = ErrorCode::ExpectedDoubleQuote;

    error_code.fmt(&mut formatter).unwrap();
    assert_eq!(formatter.output, "expected `\"`");
}

#[test]
fn test_error_code_display_eof_while_parsing_list() {
    use core::fmt::Formatter;

    struct MockFormatter {
        output: String,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter {
                output: String::new(),
            }
        }
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut formatter = MockFormatter::new();
    let error_code = ErrorCode::EofWhileParsingList;

    error_code.fmt(&mut formatter).unwrap();
    assert_eq!(formatter.output, "EOF while parsing a list");
}

#[test]
fn test_error_code_display_eof_while_parsing_object() {
    use core::fmt::Formatter;

    struct MockFormatter {
        output: String,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter {
                output: String::new(),
            }
        }
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut formatter = MockFormatter::new();
    let error_code = ErrorCode::EofWhileParsingObject;

    error_code.fmt(&mut formatter).unwrap();
    assert_eq!(formatter.output, "EOF while parsing an object");
}

#[test]
fn test_error_code_display_invalid_escape() {
    use core::fmt::Formatter;

    struct MockFormatter {
        output: String,
    }

    impl MockFormatter {
        fn new() -> Self {
            MockFormatter {
                output: String::new(),
            }
        }
    }

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.output.push_str(s);
            Ok(())
        }
    }

    let mut formatter = MockFormatter::new();
    let error_code = ErrorCode::InvalidEscape;

    error_code.fmt(&mut formatter).unwrap();
    assert_eq!(formatter.output, "invalid escape");
}


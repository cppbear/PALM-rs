// Answer 0

#[test]
fn test_display_error_code_trailing_characters() {
    use core::fmt::Formatter;

    struct MockFormatter;

    impl Formatter {
        fn new() -> Self {
            MockFormatter
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            assert_eq!(s, "trailing characters");
            Ok(())
        }
    }

    let mut formatter = MockFormatter::new();
    let error_code = ErrorCode::TrailingCharacters;
    
    let result = error_code.fmt(&mut formatter);
    assert!(result.is_ok());
}

#[test]
fn test_display_error_code_eof_while_parsing_list() {
    use core::fmt::Formatter;

    struct MockFormatter;

    impl Formatter {
        fn new() -> Self {
            MockFormatter
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            assert_eq!(s, "EOF while parsing a list");
            Ok(())
        }
    }

    let mut formatter = MockFormatter::new();
    let error_code = ErrorCode::EofWhileParsingList;

    let result = error_code.fmt(&mut formatter);
    assert!(result.is_ok());
}

#[test]
fn test_display_error_code_expected_colon() {
    use core::fmt::Formatter;

    struct MockFormatter;

    impl Formatter {
        fn new() -> Self {
            MockFormatter
        }

        fn write_str(&mut self, s: &str) -> fmt::Result {
            assert_eq!(s, "expected `:`");
            Ok(())
        }
    }

    let mut formatter = MockFormatter::new();
    let error_code = ErrorCode::ExpectedColon;

    let result = error_code.fmt(&mut formatter);
    assert!(result.is_ok());
}


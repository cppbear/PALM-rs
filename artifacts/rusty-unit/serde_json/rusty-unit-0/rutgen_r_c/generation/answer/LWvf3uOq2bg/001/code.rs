// Answer 0

#[test]
fn test_error_code_recursion_limit_exceeded_display() {
    use core::fmt::Formatter;

    struct MockFormatter;

    impl Formatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let code = ErrorCode::RecursionLimitExceeded;
    let mut formatter = MockFormatter;

    let result = code.fmt(&mut formatter);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_error_code_control_character_while_parsing_string_display() {
    use core::fmt::Formatter;

    struct MockFormatter;

    impl Formatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let code = ErrorCode::ControlCharacterWhileParsingString;
    let mut formatter = MockFormatter;

    let result = code.fmt(&mut formatter);
    assert_eq!(result, Ok(()));
}

#[test]
fn test_error_code_trailing_characters_display() {
    use core::fmt::Formatter;

    struct MockFormatter;

    impl Formatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let code = ErrorCode::TrailingCharacters;
    let mut formatter = MockFormatter;

    let result = code.fmt(&mut formatter);
    assert_eq!(result, Ok(()));
}


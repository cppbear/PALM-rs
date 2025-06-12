// Answer 0

#[test]
fn test_expected_some_value() {
    use core::fmt::Formatter;

    struct MockFormatter;

    impl Formatter {
        fn write_str(&mut self, _s: &str) -> core::fmt::Result {
            Ok(())
        }
    }

    let mut formatter = MockFormatter;
    let error_code = ErrorCode::ExpectedSomeValue;

    assert!(error_code.fmt(&mut formatter).is_ok());
}

#[test]
fn test_expected_colon() {
    use core::fmt::Formatter;

    struct MockFormatter;

    impl Formatter {
        fn write_str(&mut self, _s: &str) -> core::fmt::Result {
            Ok(())
        }
    }

    let mut formatter = MockFormatter;
    let error_code = ErrorCode::ExpectedColon;

    assert!(error_code.fmt(&mut formatter).is_ok());
} 

#[test]
fn test_eof_while_parsing_object() {
    use core::fmt::Formatter;

    struct MockFormatter;

    impl Formatter {
        fn write_str(&mut self, _s: &str) -> core::fmt::Result {
            Ok(())
        }
    }

    let mut formatter = MockFormatter;
    let error_code = ErrorCode::EofWhileParsingObject;

    assert!(error_code.fmt(&mut formatter).is_ok());
}

#[test]
fn test_io_error() {
    use core::fmt::Formatter;

    struct MockIoError;

    impl core::fmt::Display for MockIoError {
        fn fmt(&self, _f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            Ok(())
        }
    }

    struct MockFormatter;

    impl Formatter {
        fn write_str(&mut self, _s: &str) -> core::fmt::Result {
            Ok(())
        }
    }

    let mut formatter = MockFormatter;
    let error_code = ErrorCode::Io(MockIoError);

    assert!(error_code.fmt(&mut formatter).is_ok());
}


// Answer 0

#[test]
fn test_fmt_class_unclosed() {
    use std::fmt::Formatter;

    struct MockFormatter;

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let error_kind = ErrorKind::ClassUnclosed;
    let mut formatter = MockFormatter;

    assert!(error_kind.fmt(&mut formatter).is_ok());
}

#[test]
fn test_fmt_group_unclosed() {
    use std::fmt::Formatter;

    struct MockFormatter;

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let error_kind = ErrorKind::GroupUnclosed;
    let mut formatter = MockFormatter;

    assert!(error_kind.fmt(&mut formatter).is_ok());
}


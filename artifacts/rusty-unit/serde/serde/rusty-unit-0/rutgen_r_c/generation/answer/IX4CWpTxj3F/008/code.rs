// Answer 0

#[test]
fn test_fmt_seq() {
    use std::fmt;

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let unexpected_seq = Unexpected::Seq;
    let mut formatter = TestFormatter;

    let result = unexpected_seq.fmt(&mut formatter);
    assert!(result.is_ok());
}

#[test]
fn test_fmt_bool() {
    use std::fmt;

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let unexpected_bool = Unexpected::Bool(true);
    let mut formatter = TestFormatter;

    let result = unexpected_bool.fmt(&mut formatter);
    assert!(result.is_ok());
}

#[test]
fn test_fmt_unsigned() {
    use std::fmt;

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let unexpected_unsigned = Unexpected::Unsigned(42);
    let mut formatter = TestFormatter;

    let result = unexpected_unsigned.fmt(&mut formatter);
    assert!(result.is_ok());
}

#[test]
fn test_fmt_signed() {
    use std::fmt;

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let unexpected_signed = Unexpected::Signed(-7);
    let mut formatter = TestFormatter;

    let result = unexpected_signed.fmt(&mut formatter);
    assert!(result.is_ok());
}

#[test]
fn test_fmt_float() {
    use std::fmt;

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let unexpected_float = Unexpected::Float(3.14);
    let mut formatter = TestFormatter;

    let result = unexpected_float.fmt(&mut formatter);
    assert!(result.is_ok());
}

#[test]
fn test_fmt_char() {
    use std::fmt;

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let unexpected_char = Unexpected::Char('a');
    let mut formatter = TestFormatter;

    let result = unexpected_char.fmt(&mut formatter);
    assert!(result.is_ok());
}

#[test]
fn test_fmt_str() {
    use std::fmt;

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    let unexpected_str = Unexpected::Str("example string");
    let mut formatter = TestFormatter;

    let result = unexpected_str.fmt(&mut formatter);
    assert!(result.is_ok());
}


// Answer 0

#[test]
fn test_fmt_unexpected_int() {
    use serde::de::Unexpected;

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            assert_eq!(s, "not a float"); // checking the output for demonstration
            Ok(())
        }
    }

    let unexpected = Unexpected::Str("not a float");
    let json_unexpected = JsonUnexpected(unexpected);
    let mut formatter = TestFormatter;

    let result = json_unexpected.fmt(&mut formatter);
    assert!(result.is_ok());
}

#[test]
fn test_fmt_unexpected_bytes() {
    use serde::de::Unexpected;

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            assert_eq!(s, "bytes"); // checking the output for demonstration
            Ok(())
        }
    }

    let unexpected = Unexpected::Bytes(&[1, 2, 3]);
    let json_unexpected = JsonUnexpected(unexpected);
    let mut formatter = TestFormatter;

    let result = json_unexpected.fmt(&mut formatter);
    assert!(result.is_ok());
}

#[test]
fn test_fmt_unexpected_string() {
    use serde::de::Unexpected;

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            assert_eq!(s, "string"); // checking the output for demonstration
            Ok(())
        }
    }

    let unexpected = Unexpected::Str("string");
    let json_unexpected = JsonUnexpected(unexpected);
    let mut formatter = TestFormatter;

    let result = json_unexpected.fmt(&mut formatter);
    assert!(result.is_ok());
}


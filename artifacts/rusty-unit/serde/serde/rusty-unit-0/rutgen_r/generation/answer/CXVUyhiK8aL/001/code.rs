// Answer 0

#[test]
fn test_expecting() {
    use std::fmt;

    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let mut formatter = TestFormatter;
    let result = expecting(&formatter);
    assert!(result.is_ok());
}


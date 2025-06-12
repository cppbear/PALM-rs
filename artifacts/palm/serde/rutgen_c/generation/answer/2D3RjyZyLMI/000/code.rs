// Answer 0

#[test]
fn test_expectation_os_string() {
    struct MockFormatter;

    impl fmt::Write for MockFormatter {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            assert_eq!(s, "os string");
            Ok(())
        }
    }

    let mut formatter = MockFormatter;
    let visitor = OsStringVisitor;
    let result = visitor.expecting(&mut formatter);
    assert!(result.is_ok());
}


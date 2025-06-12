// Answer 0

#[test]
fn test_unexpected_newtype_struct_display() {
    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let unexpected = Unexpected::NewtypeStruct;
    let mut formatter = TestFormatter;

    // This should not panic and return Ok
    assert!(unexpected.fmt(&mut formatter).is_ok());
}

#[test]
fn test_unexpected_other_display() {
    struct TestFormatter;

    impl fmt::Write for TestFormatter {
        fn write_str(&mut self, _: &str) -> fmt::Result {
            Ok(())
        }
    }

    let unexpected = Unexpected::Other("custom message");
    let mut formatter = TestFormatter;

    // This should not panic and return Ok
    assert!(unexpected.fmt(&mut formatter).is_ok());
}


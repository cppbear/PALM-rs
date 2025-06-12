// Answer 0

#[test]
fn test_fmt() {
    use std::fmt;

    struct TestStruct;

    impl TestStruct {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            use hir::print::Printer;
            Printer::new().print(self, f)
        }
    }

    let test_instance = TestStruct;
    let mut buffer = String::new();
    let result = fmt::write(&mut buffer, |f| test_instance.fmt(f));

    assert!(result.is_ok());
    assert!(!buffer.is_empty()); // Validate that something was written to the buffer
}


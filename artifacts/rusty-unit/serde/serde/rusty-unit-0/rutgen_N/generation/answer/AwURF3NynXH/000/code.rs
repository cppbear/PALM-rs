// Answer 0

#[test]
fn test_expecting() {
    use std::fmt;

    struct MyFormatter;

    impl fmt::Write for MyFormatter {
        fn write_str(&mut self, _s: &str) -> fmt::Result {
            Ok(())
        }
    }

    struct TestStruct;

    impl TestStruct {
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("path string")
        }
    }

    let test_struct = TestStruct;
    let mut my_formatter = MyFormatter;

    let result = test_struct.expecting(&mut my_formatter);
    assert!(result.is_ok());
}


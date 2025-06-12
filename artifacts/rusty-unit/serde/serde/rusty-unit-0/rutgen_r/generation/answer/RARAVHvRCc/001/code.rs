// Answer 0

#[test]
fn test_expecting_empty_array() {
    use std::fmt;

    struct TestStruct;

    impl TestStruct {
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an empty array")
        }
    }

    let instance = TestStruct;
    let mut output = String::new();
    let result = instance.expecting(&mut fmt::Formatter::new(&mut output));

    assert!(result.is_ok());
    assert_eq!(output, "an empty array");
}


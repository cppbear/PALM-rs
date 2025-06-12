// Answer 0

#[test]
fn test_expecting() {
    use std::fmt;

    struct TestStruct;

    impl TestStruct {
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }
    }

    let test_instance = TestStruct;
    let mut output = String::new();
    let result = test_instance.expecting(&mut fmt::Formatter::new(&mut output));

    assert_eq!(result, Ok(()));
    assert_eq!(output, "a string");
}


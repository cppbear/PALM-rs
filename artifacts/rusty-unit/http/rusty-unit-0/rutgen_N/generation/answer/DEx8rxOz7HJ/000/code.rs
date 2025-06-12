// Answer 0

#[test]
fn test_fmt() {
    struct TestStruct;

    impl std::fmt::Display for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("failed to parse header value")
        }
    }

    let test_value = TestStruct;
    let mut result = String::new();
    let fmt_result = write!(&mut result, "{}", test_value);
    
    assert!(fmt_result.is_ok());
    assert_eq!(result, "failed to parse header value");
}


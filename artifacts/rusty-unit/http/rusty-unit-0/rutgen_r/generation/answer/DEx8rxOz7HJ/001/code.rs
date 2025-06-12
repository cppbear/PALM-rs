// Answer 0

#[test]
fn test_fmt() {
    struct TestHeaderValue;

    impl std::fmt::Display for TestHeaderValue {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("failed to parse header value")
        }
    }

    let header_value = TestHeaderValue;
    let mut output = String::new();
    let result = write!(&mut output, "{}", header_value);
    
    assert!(result.is_ok());
    assert_eq!(output, "failed to parse header value");
}


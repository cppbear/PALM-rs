// Answer 0

#[test]
fn test_invalid_header_value_display() {
    struct DummyInvalidHeaderValue {
        _priv: (),
    }

    impl fmt::Display for DummyInvalidHeaderValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("failed to parse header value")
        }
    }

    let invalid_header_value = DummyInvalidHeaderValue { _priv: () };
    let mut output = String::new();
    let result = write!(&mut output, "{}", invalid_header_value);
    assert!(result.is_ok());
    assert_eq!(output, "failed to parse header value");
}


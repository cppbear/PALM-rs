// Answer 0

#[test]
fn test_invalid_header_value_display() {
    struct InvalidHeaderValue {
        _priv: (),
    }

    impl fmt::Display for InvalidHeaderValue {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("failed to parse header value")
        }
    }

    let invalid_header_value = InvalidHeaderValue { _priv: () };
    let result = format!("{}", invalid_header_value);
    assert_eq!(result, "failed to parse header value");
}


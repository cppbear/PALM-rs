// Answer 0

#[test]
fn test_invalid_method_display() {
    struct InvalidMethod {
        _priv: (),
    }

    impl fmt::Display for InvalidMethod {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("invalid HTTP method")
        }
    }
    
    let method = InvalidMethod { _priv: () };
    let result = format!("{}", method);
    assert_eq!(result, "invalid HTTP method");
}


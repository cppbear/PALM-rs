// Answer 0

#[test]
fn test_fmt_invalid_http_method() {
    use std::fmt;

    struct InvalidHttpMethod;

    impl fmt::Debug for InvalidHttpMethod {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("invalid HTTP method")
        }
    }

    let method = InvalidHttpMethod;
    let mut output = String::new();
    let result = fmt::write(&mut output, format_args!("{:?}", method));

    assert!(result.is_ok());
    assert_eq!(output, "invalid HTTP method");
}


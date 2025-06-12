// Answer 0

#[test]
fn test_fmt_invalid_http_header_name() {
    use std::fmt;

    struct InvalidHeaderName;

    impl fmt::Display for InvalidHeaderName {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("invalid HTTP header name")
        }
    }

    let header_name = InvalidHeaderName;
    let mut output = String::new();
    let result = fmt::write(&mut output, format_args!("{}", header_name));
    
    assert!(result.is_ok());
    assert_eq!(output, "invalid HTTP header name");
}


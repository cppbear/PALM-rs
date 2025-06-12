// Answer 0

#[test]
fn test_fmt_invalid_status_code() {
    use std::fmt;

    struct InvalidStatusCode;

    impl fmt::Display for InvalidStatusCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("invalid status code")
        }
    }

    let status = InvalidStatusCode;
    let mut output = String::new();
    let result = write!(&mut output, "{}", status);
    
    assert!(result.is_ok());
    assert_eq!(output, "invalid status code");
}


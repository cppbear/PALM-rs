// Answer 0

#[test]
fn test_fmt_invalid_status_code() {
    struct InvalidStatusCode {
        _priv: (),
    }

    impl fmt::Display for InvalidStatusCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("invalid status code")
        }
    }

    let invalid_status = InvalidStatusCode { _priv: () };
    let mut output = String::new();
    let result = write!(&mut output, "{}", invalid_status);

    assert!(result.is_ok());
    assert_eq!(output, "invalid status code");
}


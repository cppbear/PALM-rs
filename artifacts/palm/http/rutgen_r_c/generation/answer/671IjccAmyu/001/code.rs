// Answer 0

#[test]
fn test_fmt_success() {
    struct InvalidHeaderName {
        _priv: (),
    }

    impl fmt::Display for InvalidHeaderName {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("invalid HTTP header name")
        }
    }

    let invalid_header_name = InvalidHeaderName { _priv: () };
    let mut output = String::new();
    let result = write!(&mut output, "{}", invalid_header_name);
    assert!(result.is_ok());
    assert_eq!(output, "invalid HTTP header name");
}

#[test]
#[should_panic(expected = "invalid HTTP header name")]
fn test_fmt_panic() {
    struct InvalidHeaderName {
        _priv: (),
    }

    impl fmt::Display for InvalidHeaderName {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            panic!("invalid HTTP header name")
        }
    }

    let invalid_header_name = InvalidHeaderName { _priv: () };
    let _ = format!("{}", invalid_header_name);
}

#[test]
fn test_fmt_empty_struct() {
    struct InvalidHeaderName {
        _priv: (),
    }

    impl fmt::Display for InvalidHeaderName {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("invalid HTTP header name")
        }
    }

    let invalid_header_name = InvalidHeaderName { _priv: () };
    let output = format!("{}", invalid_header_name);
    assert_eq!(output, "invalid HTTP header name");
}


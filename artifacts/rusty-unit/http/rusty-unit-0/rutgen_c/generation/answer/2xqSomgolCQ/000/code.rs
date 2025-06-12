// Answer 0

#[test]
fn test_scheme_debug_http() {
    let scheme = Scheme::HTTP;
    let mut output = vec![];
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        scheme.fmt(&mut formatter).unwrap();
    }
    assert_eq!(String::from_utf8(output).unwrap(), "http");
}

#[test]
fn test_scheme_debug_https() {
    let scheme = Scheme::HTTPS;
    let mut output = vec![];
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        scheme.fmt(&mut formatter).unwrap();
    }
    assert_eq!(String::from_utf8(output).unwrap(), "https");
}

#[test]
fn test_scheme_debug_empty() {
    let scheme = Scheme::empty();
    let mut output = vec![];
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        let result = scheme.fmt(&mut formatter);
        assert!(result.is_err());
    }
}


// Answer 0

#[test]
fn test_scheme_fmt_http() {
    let scheme = Scheme::HTTP;
    let mut output = vec![];
    let result = write!(&mut output, "{:?}", scheme);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "\"http\"");
}

#[test]
fn test_scheme_fmt_https() {
    let scheme = Scheme::HTTPS;
    let mut output = vec![];
    let result = write!(&mut output, "{:?}", scheme);
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(output).unwrap(), "\"https\"");
}

#[test]
fn test_scheme_fmt_empty() {
    let scheme = Scheme::empty();
    let mut output = vec![];
    let result = std::panic::catch_unwind(|| {
        write!(&mut output, "{:?}", scheme).unwrap();
    });
    assert!(result.is_err());
}


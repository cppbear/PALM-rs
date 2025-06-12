// Answer 0

#[test]
fn test_scheme_fmt_http() {
    let scheme = Scheme::HTTP;
    let mut output = String::new();
    let result = scheme.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "http");
}

#[test]
fn test_scheme_fmt_https() {
    let scheme = Scheme::HTTPS;
    let mut output = String::new();
    let result = scheme.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "https");
}

#[test]
fn test_scheme_fmt_empty() {
    let scheme = Scheme::empty();
    let mut output = String::new();
    // This should panic, as the `empty()` variant will reach the `unreachable!()`
    let result = std::panic::catch_unwind(|| {
        scheme.fmt(&mut output)
    });
    assert!(result.is_err());
}


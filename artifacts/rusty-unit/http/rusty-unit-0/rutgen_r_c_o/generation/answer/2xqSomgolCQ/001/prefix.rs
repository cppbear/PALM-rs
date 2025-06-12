// Answer 0

#[test]
fn test_fmt_http_scheme() {
    let scheme = Scheme::HTTP;
    let mut formatter = fmt::Formatter::new();
    let _ = scheme.fmt(&mut formatter);
}

#[test]
fn test_fmt_https_scheme() {
    let scheme = Scheme::HTTPS;
    let mut formatter = fmt::Formatter::new();
    let _ = scheme.fmt(&mut formatter);
}

#[test]
fn test_fmt_empty_scheme() {
    let scheme = Scheme::empty();
    let mut formatter = fmt::Formatter::new();
    let _ = scheme.fmt(&mut formatter);
}

#[test]
fn test_fmt_standard_scheme_custom() {
    struct CustomScheme(Box<ByteStr>);

    let custom = CustomScheme(Box::from(ByteStr::from_str("mycustomscheme").unwrap()));
    let scheme = Scheme { inner: Scheme2::Other(custom.0) };
    let mut formatter = fmt::Formatter::new();
    let _ = scheme.fmt(&mut formatter);
}

#[test]
#[should_panic]
fn test_fmt_panic_on_none_scheme() {
    let scheme = Scheme { inner: Scheme2::None };
    let mut formatter = fmt::Formatter::new();
    let _ = scheme.fmt(&mut formatter);
}

#[test]
fn test_fmt_standard_scheme_custom_length() {
    let scheme_str = "my-custom**scheme:2023"; // 24 characters
    let scheme = Scheme { inner: Scheme2::Other(Box::from(ByteStr::from_str(scheme_str).unwrap())) };
    let mut formatter = fmt::Formatter::new();
    let _ = scheme.fmt(&mut formatter);
}

#[test]
fn test_fmt_edge_case_long_scheme() {
    let scheme_str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+-.~"; // 64 characters
    let scheme = Scheme { inner: Scheme2::Other(Box::from(ByteStr::from_str(scheme_str).unwrap())) };
    let mut formatter = fmt::Formatter::new();
    let _ = scheme.fmt(&mut formatter);
}


// Answer 0

#[test]
fn test_version_http09_debug() {
    let version = Version(Http::Http09);
    let mut buffer = Vec::new();
    let result = version.fmt(&mut fmt::Formatter::new(&mut buffer));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "HTTP/0.9");
}

#[test]
fn test_version_http10_debug() {
    let version = Version(Http::Http10);
    let mut buffer = Vec::new();
    let result = version.fmt(&mut fmt::Formatter::new(&mut buffer));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "HTTP/1.0");
}

#[test]
fn test_version_http11_debug() {
    let version = Version(Http::Http11);
    let mut buffer = Vec::new();
    let result = version.fmt(&mut fmt::Formatter::new(&mut buffer));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "HTTP/1.1");
}

#[test]
fn test_version_h2_debug() {
    let version = Version(Http::H2);
    let mut buffer = Vec::new();
    let result = version.fmt(&mut fmt::Formatter::new(&mut buffer));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "HTTP/2.0");
}

#[test]
fn test_version_h3_debug() {
    let version = Version(Http::H3);
    let mut buffer = Vec::new();
    let result = version.fmt(&mut fmt::Formatter::new(&mut buffer));
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(buffer).unwrap(), "HTTP/3.0");
}


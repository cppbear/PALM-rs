// Answer 0

#[test]
fn test_version_debug_http2() {
    let version = Version(Http::H2);
    let mut output = String::new();
    let result = version.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "HTTP/2.0");
}

#[test]
fn test_version_debug_http3() {
    let version = Version(Http::H3);
    let mut output = String::new();
    let result = version.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "HTTP/3.0");
}

#[test]
fn test_version_debug_http09() {
    let version = Version(Http::Http09);
    let mut output = String::new();
    let result = version.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "HTTP/0.9");
}

#[test]
fn test_version_debug_http10() {
    let version = Version(Http::Http10);
    let mut output = String::new();
    let result = version.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "HTTP/1.0");
}

#[test]
fn test_version_debug_http11() {
    let version = Version(Http::Http11);
    let mut output = String::new();
    let result = version.fmt(&mut output);
    assert!(result.is_ok());
    assert_eq!(output, "HTTP/1.1");
}


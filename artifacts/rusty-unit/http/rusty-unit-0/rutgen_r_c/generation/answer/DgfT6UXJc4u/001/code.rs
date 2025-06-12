// Answer 0

#[test]
fn test_version_debug_http09() {
    let version = Version(Http::Http09);
    let result = format!("{:?}", version);
    assert_eq!(result, "HTTP/0.9");
}

#[test]
fn test_version_debug_http10() {
    let version = Version(Http::Http10);
    let result = format!("{:?}", version);
    assert_eq!(result, "HTTP/1.0");
}

#[test]
fn test_version_debug_http11() {
    let version = Version(Http::Http11);
    let result = format!("{:?}", version);
    assert_eq!(result, "HTTP/1.1");
}

#[test]
fn test_version_debug_h2() {
    let version = Version(Http::H2);
    let result = format!("{:?}", version);
    assert_eq!(result, "HTTP/2.0");
}

#[test]
fn test_version_debug_h3() {
    let version = Version(Http::H3);
    let result = format!("{:?}", version);
    assert_eq!(result, "HTTP/3.0");
}

#[should_panic]
#[test]
fn test_version_debug_non_exhaustive() {
    let version = Version(Http::__NonExhaustive);
    format!("{:?}", version); // This should panic
}


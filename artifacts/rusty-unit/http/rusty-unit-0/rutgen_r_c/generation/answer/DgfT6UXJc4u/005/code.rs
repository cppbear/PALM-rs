// Answer 0

#[test]
fn test_version_debug_http10() {
    struct VersionWrapper(Version);

    let version = VersionWrapper(Version(Http::Http10));
    
    let mut output = String::new();
    let result = version.0.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "HTTP/1.0");
}

#[test]
fn test_version_debug_http11() {
    struct VersionWrapper(Version);

    let version = VersionWrapper(Version(Http::Http11));
    
    let mut output = String::new();
    let result = version.0.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "HTTP/1.1");
}

#[test]
fn test_version_debug_http09() {
    struct VersionWrapper(Version);

    let version = VersionWrapper(Version(Http::Http09));
    
    let mut output = String::new();
    let result = version.0.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "HTTP/0.9");
}

#[test]
fn test_version_debug_h2() {
    struct VersionWrapper(Version);

    let version = VersionWrapper(Version(Http::H2));
    
    let mut output = String::new();
    let result = version.0.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "HTTP/2.0");
}

#[test]
fn test_version_debug_h3() {
    struct VersionWrapper(Version);

    let version = VersionWrapper(Version(Http::H3));
    
    let mut output = String::new();
    let result = version.0.fmt(&mut output);

    assert!(result.is_ok());
    assert_eq!(output, "HTTP/3.0");
}


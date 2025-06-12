// Answer 0

#[test]
fn test_port_with_valid_port() {
    let authority = Authority::from_static("example.com:8080");
    let port = authority.port().unwrap();
    assert_eq!(port.as_u16(), 8080);
    assert_eq!(port.as_str(), "8080");
}

#[test]
fn test_port_without_port() {
    let authority = Authority::from_static("example.com");
    assert!(authority.port().is_none());
}

#[test]
fn test_port_with_port_and_extra_path() {
    let authority = Authority::from_static("example.com:3000/path/to/resource");
    let port = authority.port().unwrap();
    assert_eq!(port.as_u16(), 3000);
    assert_eq!(port.as_str(), "3000");
}

#[test]
fn test_port_with_empty_string() {
    let authority = Authority::from_static("");
    assert!(authority.port().is_none());
}

#[test]
fn test_port_with_invalid_port() {
    let authority = Authority::from_static("example.com:invalid");
    assert!(authority.port().is_none());
}

#[test]
fn test_port_with_large_number() {
    let authority = Authority::from_static("example.com:65535");
    let port = authority.port().unwrap();
    assert_eq!(port.as_u16(), 65535);
    assert_eq!(port.as_str(), "65535");
}

#[test]
fn test_port_with_host_and_multiple_colons() {
    let authority = Authority::from_static("example.com:80:90");
    let port = authority.port().unwrap();
    assert_eq!(port.as_u16(), 90);
    assert_eq!(port.as_str(), "90");
}


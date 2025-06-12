// Answer 0

#[test]
fn test_host_with_registered_name() {
    let authority = Authority::from_static("example.com:80");
    assert_eq!(authority.host(), "example.com");
}

#[test]
fn test_host_with_ipv4_address() {
    let authority = Authority::from_static("192.168.1.1:8080");
    assert_eq!(authority.host(), "192.168.1.1");
}

#[test]
fn test_host_with_ipv6_address() {
    let authority = Authority::from_static("[::1]:8080");
    assert_eq!(authority.host(), "[::1]");
}

#[test]
fn test_host_with_empty_authority() {
    let authority = Authority::empty();
    assert_eq!(authority.host(), "");
}

#[test]
fn test_host_with_username_and_password() {
    let authority = Authority::from_static("username:password@hostname:80");
    assert_eq!(authority.host(), "hostname");
}

#[test]
fn test_host_with_extra_colon() {
    let authority = Authority::from_static("host:123:456");
    assert_eq!(authority.host(), "host");
}

#[test]
fn test_host_with_subdomain() {
    let authority = Authority::from_static("sub.example.com:8080");
    assert_eq!(authority.host(), "sub.example.com");
}


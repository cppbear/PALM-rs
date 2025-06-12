// Answer 0

#[test]
fn test_host_with_valid_registered_name() {
    let authority = Authority::from_static("example.com:123");
    assert_eq!(authority.host(), "example.com");
}

#[test]
fn test_host_with_ip_literal() {
    let authority = Authority::from_static("[::1]:80");
    assert_eq!(authority.host(), "[::1]");
}

#[test]
fn test_host_with_ipv4_address() {
    let authority = Authority::from_static("192.168.1.1:8080");
    assert_eq!(authority.host(), "192.168.1.1");
}

#[test]
fn test_host_with_user_info() {
    let authority = Authority::from_static("username:password@example.org:80");
    assert_eq!(authority.host(), "example.org");
}

#[test]
fn test_host_with_special_characters() {
    let authority = Authority::from_static("ex-ample.com:90");
    assert_eq!(authority.host(), "ex-ample.com");
}

#[test]
#[should_panic]
fn test_host_with_invalid_ip_literal() {
    let authority = Authority::from_static("[invalid-ip]:80");
    authority.host(); // should panic
}

#[test]
#[should_panic]
fn test_host_with_empty_authority() {
    let authority = Authority::empty();
    authority.host(); // should panic as it doesn't have a valid host
}


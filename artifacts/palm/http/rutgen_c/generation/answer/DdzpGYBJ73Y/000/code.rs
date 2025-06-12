// Answer 0

#[test]
fn test_port_u16_with_valid_port() {
    let authority = Authority::from_static("example.org:80");
    assert_eq!(authority.port_u16(), Some(80));
}

#[test]
fn test_port_u16_without_port() {
    let authority = Authority::from_static("example.org");
    assert_eq!(authority.port_u16(), None);
}

#[test]
fn test_port_u16_with_another_valid_port() {
    let authority = Authority::from_static("localhost:8080");
    assert_eq!(authority.port_u16(), Some(8080));
}

#[test]
fn test_port_u16_with_invalid_port() {
    // To create an authority that represents an invalid port scenario
    let authority = Authority::from_static("example.org:abcd");
    assert_eq!(authority.port_u16(), None);
}


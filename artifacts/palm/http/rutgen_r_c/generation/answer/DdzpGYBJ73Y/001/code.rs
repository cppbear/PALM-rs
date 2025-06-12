// Answer 0

#[test]
fn test_port_u16_with_valid_port() {
    let authority: Authority = Authority::from_static("example.org:80");
    assert_eq!(authority.port_u16(), Some(80));
}

#[test]
fn test_port_u16_without_port() {
    let authority: Authority = Authority::from_static("example.org");
    assert_eq!(authority.port_u16(), None);
}

#[test]
fn test_port_u16_with_high_port_value() {
    let authority: Authority = Authority::from_static("example.org:65535");
    assert_eq!(authority.port_u16(), Some(65535));
}

#[test]
fn test_port_u16_with_invalid_port() {
    let authority: Authority = Authority::from_static("example.org:abcd");
    assert_eq!(authority.port_u16(), None);
}

#[test]
fn test_port_u16_with_invalid_authority() {
    let authority: Authority = Authority::from_static("example.org:70000");
    assert_eq!(authority.port_u16(), None);
}


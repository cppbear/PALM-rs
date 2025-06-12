// Answer 0

#[test]
fn test_port_u16_with_valid_port() {
    let authority = Authority::from_static("example.org:80");
    authority.port_u16();
}

#[test]
fn test_port_u16_with_lowest_valid_port() {
    let authority = Authority::from_static("example.org:0");
    authority.port_u16();
}

#[test]
fn test_port_u16_with_highest_valid_port() {
    let authority = Authority::from_static("example.org:65535");
    authority.port_u16();
}

#[test]
fn test_port_u16_without_port() {
    let authority = Authority::from_static("example.org");
    authority.port_u16();
}

#[test]
fn test_port_u16_with_invalid_port() {
    let authority = Authority::from_static("example.org:abcd");
    authority.port_u16();
}

#[test]
fn test_port_u16_with_minimal_port_after_colon() {
    let authority = Authority::from_static("example.org:1");
    authority.port_u16();
}

#[test]
fn test_port_u16_with_maximal_port_before_unexpected_characters() {
    let authority = Authority::from_static("example.org:65535#");
    authority.port_u16();
}

#[test]
fn test_port_u16_with_edge_case_ports() {
    // Testing with port 1 (lowest)
    let authority = Authority::from_static("example.org:1");
    authority.port_u16();
    
    // Testing with port 65534 (highest valid port - 1)
    let authority = Authority::from_static("example.org:65534");
    authority.port_u16();
}


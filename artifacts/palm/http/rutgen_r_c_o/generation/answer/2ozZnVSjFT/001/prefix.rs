// Answer 0

#[test]
fn test_host_example_com() {
    let authority = Authority::from_static("example.com");
    let _ = authority.host();
}

#[test]
fn test_host_localhost() {
    let authority = Authority::from_static("localhost");
    let _ = authority.host();
}

#[test]
fn test_host_ipv4() {
    let authority = Authority::from_static("192.168.1.1");
    let _ = authority.host();
}

#[test]
fn test_host_ipv6() {
    let authority = Authority::from_static("[::1]");
    let _ = authority.host();
}

#[test]
fn test_host_ip_address() {
    let authority = Authority::from_static("123.45.67.89");
    let _ = authority.host();
}

#[test]
fn test_host_with_user_info() {
    let authority = Authority::from_static("user@host.com:8080");
    let _ = authority.host();
}

#[test]
fn test_host_localhost_with_port() {
    let authority = Authority::from_static("localhost:80");
    let _ = authority.host();
}

#[test]
fn test_host_ipv6_full() {
    let authority = Authority::from_static("[2001:db8::1]");
    let _ = authority.host();
}


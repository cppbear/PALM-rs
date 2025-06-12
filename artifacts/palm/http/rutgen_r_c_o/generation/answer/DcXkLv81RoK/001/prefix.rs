// Answer 0

#[test]
fn test_port_with_valid_port() {
    let authority: Authority = Authority::from_static("example.org:80");
    let port = authority.port();
}

#[test]
fn test_port_with_another_valid_port() {
    let authority: Authority = Authority::from_static("example.com:443");
    let port = authority.port();
}

#[test]
fn test_port_with_maximum_valid_port() {
    let authority: Authority = Authority::from_static("example.net:65535");
    let port = authority.port();
}

#[test]
fn test_port_without_port() {
    let authority: Authority = Authority::from_static("example.org");
    let port = authority.port();
}

#[test]
fn test_port_with_invalid_port() {
    let authority: Authority = Authority::from_static("example.org:invalid");
    let port = authority.port();
}

#[test]
fn test_port_with_port_zero() {
    let authority: Authority = Authority::from_static("example.org:0");
    let port = authority.port();
}

#[test]
fn test_port_with_extra_colon() {
    let authority: Authority = Authority::from_static("example.org::8080");
    let port = authority.port();
}


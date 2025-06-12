// Answer 0

#[derive(Debug)]
struct Port {
    port: u16,
    repr: String,
}

#[derive(Debug)]
struct InvalidUri;

#[derive(Debug)]
struct ErrorKind;

impl ErrorKind {
    fn invalid_port() -> InvalidUri {
        InvalidUri
    }
}

pub(crate) fn from_str(bytes: &str) -> Result<Port, InvalidUri> {
    bytes
        .parse::<u16>()
        .map(|port| Port { port, repr: bytes.to_string() })
        .map_err(|_| ErrorKind::invalid_port())
}

#[test]
fn test_from_str_valid_port() {
    let result = from_str("8080");
    assert!(result.is_ok());
    let port = result.unwrap();
    assert_eq!(port.port, 8080);
    assert_eq!(port.repr, "8080");
}

#[test]
fn test_from_str_invalid_port_non_numeric() {
    let result = from_str("abc");
    assert!(result.is_err());
}

#[test]
fn test_from_str_invalid_port_out_of_range() {
    let result = from_str("70000");
    assert!(result.is_err());
}

#[test]
fn test_from_str_boundary_port_min() {
    let result = from_str("1");
    assert!(result.is_ok());
    let port = result.unwrap();
    assert_eq!(port.port, 1);
}

#[test]
fn test_from_str_boundary_port_max() {
    let result = from_str("65535");
    assert!(result.is_ok());
    let port = result.unwrap();
    assert_eq!(port.port, 65535);
}


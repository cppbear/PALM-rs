// Answer 0

#[test]
fn test_port_as_str_valid() {
    let port = Port::from_str("80").unwrap();
    port.as_str();
}

#[test]
fn test_port_as_str_minimum() {
    let port = Port::from_str("0").unwrap();
    port.as_str();
}

#[test]
fn test_port_as_str_maximum() {
    let port = Port::from_str("65535").unwrap();
    port.as_str();
}

#[test]
fn test_port_as_str_non_numeric() {
    let result = Port::from_str("invalid");
    assert!(result.is_err());
}

#[test]
fn test_port_as_str_negative_number() {
    let result = Port::from_str("-1");
    assert!(result.is_err());
}

#[test]
fn test_port_as_str_above_maximum() {
    let result = Port::from_str("65536");
    assert!(result.is_err());
}

#[test]
fn test_port_as_str_non_decimal() {
    let result = Port::from_str("abc");
    assert!(result.is_err());
}


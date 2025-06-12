// Answer 0

#[test]
fn test_from_str_valid_port() {
    let valid_input = "8080";
    let result = from_str(valid_input);
    assert!(result.is_ok());
    if let Ok(port) = result {
        assert_eq!(port.port, 8080);
        assert_eq!(port.repr, valid_input);
    }
}

#[test]
fn test_from_str_minimum_port() {
    let valid_input = "1";
    let result = from_str(valid_input);
    assert!(result.is_ok());
    if let Ok(port) = result {
        assert_eq!(port.port, 1);
        assert_eq!(port.repr, valid_input);
    }
}

#[test]
fn test_from_str_maximum_port() {
    let valid_input = "65535";
    let result = from_str(valid_input);
    assert!(result.is_ok());
    if let Ok(port) = result {
        assert_eq!(port.port, 65535);
        assert_eq!(port.repr, valid_input);
    }
}

#[test]
#[should_panic]
fn test_from_str_invalid_port_non_numeric() {
    let invalid_input = "abc";
    let _ = from_str(invalid_input); // Should panic
}

#[test]
#[should_panic]
fn test_from_str_invalid_port_out_of_range_high() {
    let invalid_input = "70000";
    let _ = from_str(invalid_input); // Should panic
}

#[test]
#[should_panic]
fn test_from_str_invalid_port_out_of_range_low() {
    let invalid_input = "-1";
    let _ = from_str(invalid_input); // Should panic
}

#[test]
fn test_from_str_empty_string() {
    let invalid_input = "";
    let result = from_str(invalid_input);
    assert!(result.is_err());
}


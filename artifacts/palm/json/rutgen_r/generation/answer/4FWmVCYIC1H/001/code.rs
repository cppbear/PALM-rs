// Answer 0

#[test]
fn test_serialize_u16_valid() {
    let value: u16 = 12345;
    let result = serialize_u16(value);
    assert_eq!(result, Ok("12345".to_string()));
}

#[test]
fn test_serialize_u16_zero() {
    let value: u16 = 0;
    let result = serialize_u16(value);
    assert_eq!(result, Ok("0".to_string()));
}

#[test]
fn test_serialize_u16_max() {
    let value: u16 = u16::MAX;
    let result = serialize_u16(value);
    assert_eq!(result, Ok("65535".to_string()));
}


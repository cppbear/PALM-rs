// Answer 0

#[test]
fn test_serialize_u8_zero() {
    let result = serialize_u8(0u8);
    assert_eq!(result, Ok("0".to_string()));
}

#[test]
fn test_serialize_u8_max() {
    let result = serialize_u8(255u8);
    assert_eq!(result, Ok("255".to_string()));
}

#[test]
fn test_serialize_u8_half() {
    let result = serialize_u8(128u8);
    assert_eq!(result, Ok("128".to_string()));
}

#[test]
fn test_serialize_u8_one() {
    let result = serialize_u8(1u8);
    assert_eq!(result, Ok("1".to_string()));
}

#[test]
fn test_serialize_u8_five() {
    let result = serialize_u8(5u8);
    assert_eq!(result, Ok("5".to_string()));
}


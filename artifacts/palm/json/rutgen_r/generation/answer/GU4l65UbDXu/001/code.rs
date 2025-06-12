// Answer 0

#[test]
fn test_serialize_u8_zero() {
    let result = serialize_u8(0);
    assert_eq!(result.unwrap(), "0");
}

#[test]
fn test_serialize_u8_max() {
    let result = serialize_u8(255);
    assert_eq!(result.unwrap(), "255");
}

#[test]
fn test_serialize_u8_middle() {
    let result = serialize_u8(128);
    assert_eq!(result.unwrap(), "128");
}

#[test]
fn test_serialize_u8_one() {
    let result = serialize_u8(1);
    assert_eq!(result.unwrap(), "1");
}

#[test]
fn test_serialize_u8_ten() {
    let result = serialize_u8(10);
    assert_eq!(result.unwrap(), "10");
}


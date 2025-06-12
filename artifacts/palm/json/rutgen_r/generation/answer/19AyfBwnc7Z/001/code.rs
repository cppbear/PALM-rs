// Answer 0

#[test]
fn test_serialize_i16_positive() {
    let value: i16 = 1234;
    let result = serialize_i16(value);
    assert_eq!(result.unwrap(), "1234");
}

#[test]
fn test_serialize_i16_negative() {
    let value: i16 = -1234;
    let result = serialize_i16(value);
    assert_eq!(result.unwrap(), "-1234");
}

#[test]
fn test_serialize_i16_zero() {
    let value: i16 = 0;
    let result = serialize_i16(value);
    assert_eq!(result.unwrap(), "0");
}

#[test]
fn test_serialize_i16_min() {
    let value: i16 = i16::MIN;
    let result = serialize_i16(value);
    assert_eq!(result.unwrap(), "-32768");
}

#[test]
fn test_serialize_i16_max() {
    let value: i16 = i16::MAX;
    let result = serialize_i16(value);
    assert_eq!(result.unwrap(), "32767");
}


// Answer 0

#[test]
fn test_serialize_i16_positive_value() {
    let value: i16 = 1234;
    let result = serialize_i16(value);
    assert_eq!(result, Ok("1234".to_owned()));
}

#[test]
fn test_serialize_i16_negative_value() {
    let value: i16 = -567;
    let result = serialize_i16(value);
    assert_eq!(result, Ok("-567".to_owned()));
}

#[test]
fn test_serialize_i16_zero_value() {
    let value: i16 = 0;
    let result = serialize_i16(value);
    assert_eq!(result, Ok("0".to_owned()));
}

#[test]
fn test_serialize_i16_min_value() {
    let value: i16 = i16::MIN; // -32768
    let result = serialize_i16(value);
    assert_eq!(result, Ok("-32768".to_owned()));
}

#[test]
fn test_serialize_i16_max_value() {
    let value: i16 = i16::MAX; // 32767
    let result = serialize_i16(value);
    assert_eq!(result, Ok("32767".to_owned()));
}


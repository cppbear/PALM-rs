// Answer 0

#[test]
fn test_serialize_i16_positive() {
    let value: i16 = 12345;
    let result = serialize_i16(value);
    assert_eq!(result.unwrap(), "12345");
}

#[test]
fn test_serialize_i16_negative() {
    let value: i16 = -12345;
    let result = serialize_i16(value);
    assert_eq!(result.unwrap(), "-12345");
}

#[test]
fn test_serialize_i16_zero() {
    let value: i16 = 0;
    let result = serialize_i16(value);
    assert_eq!(result.unwrap(), "0");
} 

#[test]
#[should_panic]
fn test_serialize_i16_overflow() {
    let value: i16 = 32768; // this will cause i16 overflow
    let _ = serialize_i16(value);
} 

#[test]
#[should_panic]
fn test_serialize_i16_underflow() {
    let value: i16 = -32769; // this will cause i16 underflow
    let _ = serialize_i16(value);
}


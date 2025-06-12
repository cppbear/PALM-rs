// Answer 0

#[test]
fn test_serialize_i128_positive() {
    let value: i128 = 12345678901234567890;
    let result = serialize_i128(value);
    assert_eq!(result.unwrap(), "12345678901234567890");
}

#[test]
fn test_serialize_i128_negative() {
    let value: i128 = -12345678901234567890;
    let result = serialize_i128(value);
    assert_eq!(result.unwrap(), "-12345678901234567890");
}

#[test]
fn test_serialize_i128_zero() {
    let value: i128 = 0;
    let result = serialize_i128(value);
    assert_eq!(result.unwrap(), "0");
}

#[test]
#[should_panic]
fn test_serialize_i128_overflow() {
    let value: i128 = i128::MAX + 1; // This should cause an overflow
    serialize_i128(value).unwrap();
}


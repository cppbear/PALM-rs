// Answer 0

#[test]
fn test_serialize_u128_zero() {
    let value: u128 = 0;
    let result = serialize_u128(value);
    assert_eq!(result, Ok("0".to_owned()));
}

#[test]
fn test_serialize_u128_small_value() {
    let value: u128 = 42;
    let result = serialize_u128(value);
    assert_eq!(result, Ok("42".to_owned()));
}

#[test]
fn test_serialize_u128_large_value() {
    let value: u128 = 1_000_000_000_000_000_000_000;
    let result = serialize_u128(value);
    assert_eq!(result, Ok("1000000000000000000".to_owned()));
}

#[test]
fn test_serialize_u128_max_value() {
    let value: u128 = u128::MAX;
    let result = serialize_u128(value);
    assert_eq!(result, Ok("340282366920938463463374607431768211455".to_owned()));
}


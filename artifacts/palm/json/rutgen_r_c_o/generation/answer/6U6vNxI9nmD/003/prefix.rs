// Answer 0

#[test]
fn test_as_u64_positive_integer_0() {
    let number = Number::from_u64(0).unwrap();
    number.as_u64();
}

#[test]
fn test_as_u64_positive_integer_1() {
    let number = Number::from_u64(1).unwrap();
    number.as_u64();
}

#[test]
fn test_as_u64_positive_integer_max() {
    let number = Number::from_u64(18446744073709551615).unwrap();
    number.as_u64();
}

#[test]
fn test_as_u64_positive_integer_mid() {
    let number = Number::from_u64(9223372036854775807).unwrap();
    number.as_u64();
}


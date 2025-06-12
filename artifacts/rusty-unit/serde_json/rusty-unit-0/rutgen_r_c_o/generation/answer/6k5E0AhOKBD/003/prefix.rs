// Answer 0

#[test]
fn test_is_u64_positive_integer() {
    let number = Number::from_u128(0).unwrap(); // 0
    number.is_u64();

    let number = Number::from_u128(1).unwrap(); // 1
    number.is_u64();

    let number = Number::from_u128(123456789).unwrap(); // 123456789
    number.is_u64();

    let number = Number::from_u128(18446744073709551615).unwrap(); // u64::MAX
    number.is_u64();
}

#[test]
fn test_is_u64_boundary_conditions() {
    let number = Number::from_u128(10).unwrap(); // Within valid range
    number.is_u64();

    let number = Number::from_u128(100).unwrap(); // Within valid range
    number.is_u64();

    let number = Number::from_u128(18446744073709551614).unwrap(); // Just below u64::MAX
    number.is_u64();
}


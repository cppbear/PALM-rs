// Answer 0

#[test]
fn test_as_u128_positive_integer_0() {
    let number = Number::from_u64(0).unwrap();
    let _ = number.as_u128();
}

#[test]
fn test_as_u128_positive_integer_max() {
    let number = Number::from_u64(18_446_744_073_709_551_615).unwrap();
    let _ = number.as_u128();
}

#[test]
fn test_as_u128_positive_integer_1() {
    let number = Number::from_u64(1).unwrap();
    let _ = number.as_u128();
}

#[test]
fn test_as_u128_positive_integer_123456() {
    let number = Number::from_u64(123456).unwrap();
    let _ = number.as_u128();
}

#[test]
fn test_as_u128_positive_integer_999999999999999999() {
    let number = Number::from_u64(999999999999999999).unwrap();
    let _ = number.as_u128();
}


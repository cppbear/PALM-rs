// Answer 0

#[test]
fn test_as_i128_positive_int_zero() {
    let number = Number::from(0u64);
    let _ = number.as_i128();
}

#[test]
fn test_as_i128_positive_int_small_value() {
    let number = Number::from(5u64);
    let _ = number.as_i128();
}

#[test]
fn test_as_i128_positive_int_large_value() {
    let number = Number::from(12345678901234567890u64);
    let _ = number.as_i128();
}

#[test]
fn test_as_i128_positive_int_max_value() {
    let number = Number::from(u64::MAX);
    let _ = number.as_i128();
}


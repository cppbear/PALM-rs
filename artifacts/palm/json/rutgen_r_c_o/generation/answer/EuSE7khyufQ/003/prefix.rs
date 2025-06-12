// Answer 0

#[test]
fn test_unexpected_positive_integer_min() {
    let number = Number { n: N::PosInt(0) };
    let _ = number.unexpected();
}

#[test]
fn test_unexpected_positive_integer_mid() {
    let number = Number { n: N::PosInt(123456789) };
    let _ = number.unexpected();
}

#[test]
fn test_unexpected_positive_integer_max() {
    let number = Number { n: N::PosInt(u64::MAX) };
    let _ = number.unexpected();
}


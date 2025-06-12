// Answer 0

#[test]
fn test_as_u128_float_negative() {
    let number = Number { n: N::Float(-1.23) };
    let _ = number.as_u128();
}

#[test]
fn test_as_u128_float_zero() {
    let number = Number { n: N::Float(0.0) };
    let _ = number.as_u128();
}

#[test]
fn test_as_u128_float_positive() {
    let number = Number { n: N::Float(4.56) };
    let _ = number.as_u128();
}


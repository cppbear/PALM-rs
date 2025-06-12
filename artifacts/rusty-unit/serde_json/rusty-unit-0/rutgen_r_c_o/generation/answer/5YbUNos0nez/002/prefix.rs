// Answer 0

#[test]
fn test_as_i128_negative_min() {
    let number = Number::from(-9223372036854775808i64);
    let result = number.as_i128();
}

#[test]
fn test_as_i128_negative_one() {
    let number = Number::from(-1i64);
    let result = number.as_i128();
}

#[test]
fn test_as_i128_negative_middle() {
    let number = Number::from(-123456789i64);
    let result = number.as_i128();
}

#[test]
fn test_as_i128_negative_five() {
    let number = Number::from(-5i64);
    let result = number.as_i128();
}


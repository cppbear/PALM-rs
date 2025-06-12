// Answer 0

#[test]
fn test_number_as_i64_positive_int() {
    let number = Number { n: N::PosInt(42) };
    assert_eq!(number.as_i64(), Some(42));
}

#[test]
fn test_number_as_i64_negative_int() {
    let number = Number { n: N::NegInt(-42) };
    assert_eq!(number.as_i64(), Some(-42));
}

#[test]
fn test_number_as_i64_float() {
    let number = Number { n: N::Float(3.14) };
    assert_eq!(number.as_i64(), None);
}

#[test]
fn test_number_as_i64_too_large() {
    let number = Number { n: N::PosInt(u64::MAX) };
    assert_eq!(number.as_i64(), None);
}


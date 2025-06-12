// Answer 0

#[test]
fn test_as_f64_positive_integer() {
    let number = Number { n: N::PosInt(42) };
    assert_eq!(number.as_f64(), Some(42.0));
}

#[test]
fn test_as_f64_negative_integer() {
    let number = Number { n: N::NegInt(-42) };
    assert_eq!(number.as_f64(), Some(-42.0));
}

#[test]
fn test_as_f64_float() {
    let number = Number { n: N::Float(3.14) };
    assert_eq!(number.as_f64(), Some(3.14));
}

#[test]
fn test_as_f64_large_integer() {
    let number = Number { n: N::PosInt(u64::MAX) };
    assert_eq!(number.as_f64(), Some(u64::MAX as f64));
}

#[test]
fn test_as_f64_small_negative_integer() {
    let number = Number { n: N::NegInt(i64::MIN) };
    assert_eq!(number.as_f64(), Some(i64::MIN as f64));
}


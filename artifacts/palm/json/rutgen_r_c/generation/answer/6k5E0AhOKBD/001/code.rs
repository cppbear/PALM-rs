// Answer 0

#[test]
fn test_is_u64_with_negative_float() {
    let number = Number { n: N::Float(-3.14) };
    assert_eq!(number.is_u64(), false);
}

#[test]
fn test_is_u64_with_positive_float() {
    let number = Number { n: N::Float(3.14) };
    assert_eq!(number.is_u64(), false);
}

#[test]
fn test_is_u64_with_negative_integer() {
    let number = Number { n: N::NegInt(-5) };
    assert_eq!(number.is_u64(), false);
}

#[test]
fn test_is_u64_with_positive_integer() {
    let number = Number { n: N::PosInt(10) };
    assert_eq!(number.is_u64(), true);
}

#[test]
fn test_is_u64_with_large_number() {
    let number = Number { n: N::PosInt(u64::MAX) };
    assert_eq!(number.is_u64(), true);
} 

#[test]
fn test_is_u64_with_float_zero() {
    let number = Number { n: N::Float(0.0) };
    assert_eq!(number.is_u64(), false);
} 

#[test]
fn test_is_u64_with_float_infinity() {
    let number = Number { n: N::Float(f64::INFINITY) };
    assert_eq!(number.is_u64(), false);
} 

#[test]
fn test_is_u64_with_float_nan() {
    let number = Number { n: N::Float(f64::NAN) };
    assert_eq!(number.is_u64(), false);
}


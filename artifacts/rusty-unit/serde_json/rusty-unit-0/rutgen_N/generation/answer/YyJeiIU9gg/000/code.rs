// Answer 0

#[test]
fn test_from_u128_within_limit() {
    let number = from_u128(256);
    assert!(number.is_some());
    assert_eq!(number.unwrap().n, Number::PosInt(256));
}

#[test]
fn test_from_u128_exact_limit() {
    let number = from_u128(u64::MAX);
    assert!(number.is_some());
    assert_eq!(number.unwrap().n, Number::PosInt(u64::MAX));
}

#[test]
fn test_from_u128_above_limit() {
    let number = from_u128(u64::MAX as u128 + 1);
    #[cfg(not(feature = "arbitrary_precision"))]
    {
        assert!(number.is_none());
    }
    #[cfg(feature = "arbitrary_precision")]
    {
        assert!(number.is_some());
        assert_eq!(number.unwrap().n, "18446744073709551617");
    }
}

#[test]
fn test_from_u128_zero() {
    let number = from_u128(0);
    assert!(number.is_some());
    assert_eq!(number.unwrap().n, Number::PosInt(0));
}


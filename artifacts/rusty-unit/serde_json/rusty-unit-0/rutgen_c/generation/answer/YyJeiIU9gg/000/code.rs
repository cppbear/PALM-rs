// Answer 0

#[test]
fn test_from_u128_within_u64_range() {
    let num = Number::from_u128(256);
    assert!(num.is_some());
    if let Some(n) = num {
        assert!(n.is_u64());
        assert_eq!(n.as_u64(), Some(256));
    }
}

#[test]
fn test_from_u128_exceeding_u64_max() {
    let num = Number::from_u128(u128::MAX); // This will only succeed if arbitrary_precision is enabled
    #[cfg(not(feature = "arbitrary_precision"))]
    assert!(num.is_none());
    #[cfg(feature = "arbitrary_precision")]
    {
        assert!(num.is_some());
        if let Some(n) = num {
            assert_eq!(n.as_str(), "340282366920938463463374607431768211455");
        }
    }
}

#[test]
fn test_from_u128_zero() {
    let num = Number::from_u128(0);
    assert!(num.is_some());
    if let Some(n) = num {
        assert!(n.is_u64());
        assert_eq!(n.as_u64(), Some(0));
    }
}

#[test]
fn test_from_u128_one() {
    let num = Number::from_u128(1);
    assert!(num.is_some());
    if let Some(n) = num {
        assert!(n.is_u64());
        assert_eq!(n.as_u64(), Some(1));
    }
}

#[test]
fn test_from_u128_huge_number() {
    let num = Number::from_u128(99999999999999999999999999999999999999); // Example large number
    #[cfg(not(feature = "arbitrary_precision"))]
    assert!(num.is_none());
    #[cfg(feature = "arbitrary_precision")]
    {
        assert!(num.is_some());
        if let Some(n) = num {
            assert_eq!(n.as_str(), "99999999999999999999999999999999999999");
        }
    }
}


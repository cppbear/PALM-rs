// Answer 0

#[test]
fn test_is_i64_with_negative_integer() {
    struct NegativeInteger(i64);

    impl From<NegativeInteger> for Number {
        fn from(neg: NegativeInteger) -> Self {
            Number { n: N::NegInt(neg.0) }
        }
    }

    let negative_num = NegativeInteger(-1);
    let number: Number = negative_num.into();
    
    assert!(number.is_i64());
}

#[test]
fn test_is_i64_with_zero() {
    struct Zero;

    impl From<Zero> for Number {
        fn from(_: Zero) -> Self {
            Number { n: N::PosInt(0) }
        }
    }

    let zero = Zero;
    let number: Number = zero.into();

    assert!(number.is_i64());
}

#[test]
fn test_is_i64_with_large_negative_integer() {
    struct LargeNegativeInteger(i64);

    impl From<LargeNegativeInteger> for Number {
        fn from(neg: LargeNegativeInteger) -> Self {
            Number { n: N::NegInt(neg.0) }
        }
    }

    let large_negative = LargeNegativeInteger(-i64::MAX);
    let number: Number = large_negative.into();

    assert!(number.is_i64());
}


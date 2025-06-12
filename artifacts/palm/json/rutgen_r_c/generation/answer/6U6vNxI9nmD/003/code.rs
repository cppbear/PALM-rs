// Answer 0

#[test]
fn test_as_u64_positive_integer() {
    struct PositiveInteger {
        n: N,
    }

    impl PositiveInteger {
        fn new(value: u64) -> Self {
            PositiveInteger {
                n: N::PosInt(value),
            }
        }

        pub fn as_u64(&self) -> Option<u64> {
            match self.n {
                N::PosInt(n) => Some(n),
                N::NegInt(_) | N::Float(_) => None,
            }
        }
    }

    let number = PositiveInteger::new(42);
    assert_eq!(number.as_u64(), Some(42));
}

#[test]
fn test_as_u64_negative_integer() {
    struct NegativeInteger {
        n: N,
    }

    impl NegativeInteger {
        fn new(value: i64) -> Self {
            NegativeInteger {
                n: if value < 0 { N::NegInt(value) } else { N::PosInt(value as u64) },
            }
        }

        pub fn as_u64(&self) -> Option<u64> {
            match self.n {
                N::PosInt(n) => Some(n),
                N::NegInt(_) | N::Float(_) => None,
            }
        }
    }

    let number = NegativeInteger::new(-1);
    assert_eq!(number.as_u64(), None);
}

#[test]
fn test_as_u64_float() {
    struct FloatNumber {
        n: N,
    }

    impl FloatNumber {
        fn new(value: f64) -> Self {
            FloatNumber {
                n: N::Float(value),
            }
        }

        pub fn as_u64(&self) -> Option<u64> {
            match self.n {
                N::PosInt(n) => Some(n),
                N::NegInt(_) | N::Float(_) => None,
            }
        }
    }

    let number = FloatNumber::new(3.14);
    assert_eq!(number.as_u64(), None);
}


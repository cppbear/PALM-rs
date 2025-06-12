// Answer 0

#[test]
fn test_as_u128_with_negative_integer() {
    struct TestNumber {
        n: N,
    }

    impl Number {
        pub fn as_u128(&self) -> Option<u128> {
            #[cfg(not(feature = "arbitrary_precision"))]
            match self.n {
                N::PosInt(n) => Some(n as u128),
                N::NegInt(_) | N::Float(_) => None,
            }
            #[cfg(feature = "arbitrary_precision")]
            self.n.parse().ok()
        }
    }

    let number = TestNumber { n: N::NegInt(-42) };
    assert_eq!(number.as_u128(), None);
}

#[test]
fn test_as_u128_with_float() {
    struct TestNumber {
        n: N,
    }

    impl Number {
        pub fn as_u128(&self) -> Option<u128> {
            #[cfg(not(feature = "arbitrary_precision"))]
            match self.n {
                N::PosInt(n) => Some(n as u128),
                N::NegInt(_) | N::Float(_) => None,
            }
            #[cfg(feature = "arbitrary_precision")]
            self.n.parse().ok()
        }
    }

    let number = TestNumber { n: N::Float(3.14) };
    assert_eq!(number.as_u128(), None);
}


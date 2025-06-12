// Answer 0

#[test]
fn test_as_u128_float_case() {
    struct N {
        n: NumberType,
    }

    enum NumberType {
        PosInt(u128),
        NegInt(i128),
        Float(f64),
    }

    struct Number {
        n: NumberType,
    }

    impl Number {
        pub fn as_u128(&self) -> Option<u128> {
            #[cfg(not(feature = "arbitrary_precision"))]
            match self.n {
                NumberType::PosInt(n) => Some(n as u128),
                NumberType::NegInt(_) | NumberType::Float(_) => None,
            }
            #[cfg(feature = "arbitrary_precision")]
            self.n.parse().ok()
        }
    }

    let number = Number {
        n: NumberType::Float(3.14),
    };

    assert_eq!(number.as_u128(), None);
}


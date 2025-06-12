// Answer 0

#[test]
fn test_is_f64_with_float() {
    struct Number {
        n: N,
    }

    enum N {
        Float(f64),
        PosInt(u64),
        NegInt(i64),
    }

    let number = Number {
        n: N::Float(3.14),
    };
    assert!(number.is_f64());
}

#[test]
fn test_is_f64_with_positive_integer() {
    struct Number {
        n: N,
    }

    enum N {
        Float(f64),
        PosInt(u64),
        NegInt(i64),
    }

    let number = Number {
        n: N::PosInt(42),
    };
    assert!(!number.is_f64());
}

#[test]
fn test_is_f64_with_negative_integer() {
    struct Number {
        n: N,
    }

    enum N {
        Float(f64),
        PosInt(u64),
        NegInt(i64),
    }

    let number = Number {
        n: N::NegInt(-42),
    };
    assert!(!number.is_f64());
}

#[test]
fn test_is_f64_with_arbitrary_precision_float() {
    struct Number {
        n: N,
    }

    enum N {
        Float(f64),
        PosInt(u64),
        NegInt(i64),
        ArbitraryPrecision(String),
    }

    impl Number {
        #[cfg(not(feature = "arbitrary_precision"))]
        pub fn is_f64(&self) -> bool {
            match self.n {
                N::Float(_) => true,
                N::PosInt(_) | N::NegInt(_) => false,
            }
        }

        #[cfg(feature = "arbitrary_precision")]
        pub fn is_f64(&self) -> bool {
            if let N::ArbitraryPrecision(ref s) = self.n {
                s.chars().any(|c| c == '.' || c == 'e' || c == 'E') &&
                s.parse::<f64>().ok().map_or(false, f64::is_finite)
            } else {
                false
            }
        }
    }

    let number = Number {
        n: N::ArbitraryPrecision("2.71828".to_string()),
    };
    assert!(number.is_f64());
}


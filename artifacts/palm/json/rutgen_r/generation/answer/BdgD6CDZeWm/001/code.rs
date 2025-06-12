// Answer 0

#[test]
fn test_as_f64_float() {
    struct N {
        float: f64,
    }

    struct Number {
        n: N,
    }

    impl Number {
        pub fn as_f64(&self) -> Option<f64> {
            #[cfg(not(feature = "arbitrary_precision"))]
            match self.n.float {
                n if n.is_finite() => Some(n),
                _ => None,
            }
            #[cfg(feature = "arbitrary_precision")]
            self.n.float.parse::<f64>().ok().filter(|float| float.is_finite())
        }
    }

    let number = Number { n: N { float: 3.14 } };
    assert_eq!(number.as_f64(), Some(3.14));

    let number_neg = Number { n: N { float: -2.71 } };
    assert_eq!(number_neg.as_f64(), Some(-2.71));

    let number_zero = Number { n: N { float: 0.0 } };
    assert_eq!(number_zero.as_f64(), Some(0.0));
}


// Answer 0

#[test]
fn test_as_f64_neg_int() {
    struct N {
        n: NegInt,
    }
    
    struct NegInt(i32);

    impl N {
        pub fn as_f64(&self) -> Option<f64> {
            #[cfg(not(feature = "arbitrary_precision"))]
            match &self.n {
                NegInt(n) => Some(*n as f64),
            }
            #[cfg(feature = "arbitrary_precision")]
            self.n.parse::<f64>().ok().filter(|float| float.is_finite())
        }
    }

    let negative_integer = NegInt(-42);
    let n = N { n: negative_integer };
    let result = n.as_f64();
    assert_eq!(result, Some(-42.0));
}

#[test]
fn test_as_f64_neg_int_zero() {
    struct N {
        n: NegInt,
    }
    
    struct NegInt(i32);

    impl N {
        pub fn as_f64(&self) -> Option<f64> {
            #[cfg(not(feature = "arbitrary_precision"))]
            match &self.n {
                NegInt(n) => Some(*n as f64),
            }
            #[cfg(feature = "arbitrary_precision")]
            self.n.parse::<f64>().ok().filter(|float| float.is_finite())
        }
    }

    let negative_integer_zero = NegInt(0);
    let n = N { n: negative_integer_zero };
    let result = n.as_f64();
    assert_eq!(result, Some(0.0));
}

#[test]
fn test_as_f64_neg_int_large() {
    struct N {
        n: NegInt,
    }
    
    struct NegInt(i32);

    impl N {
        pub fn as_f64(&self) -> Option<f64> {
            #[cfg(not(feature = "arbitrary_precision"))]
            match &self.n {
                NegInt(n) => Some(*n as f64),
            }
            #[cfg(feature = "arbitrary_precision")]
            self.n.parse::<f64>().ok().filter(|float| float.is_finite())
        }
    }

    let negative_integer_large = NegInt(-100000);
    let n = N { n: negative_integer_large };
    let result = n.as_f64();
    assert_eq!(result, Some(-100000.0));
}


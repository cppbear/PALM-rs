// Answer 0

#[test]
fn test_as_f32_with_positive_float() {
    struct N {
        n: f64,
    };

    impl N {
        fn as_f32(&self) -> Option<f32> {
            #[cfg(not(feature = "arbitrary_precision"))]
            match self.n {
                n if n.is_sign_positive() => Some(n as f32),
                _ => None,
            }
            #[cfg(feature = "arbitrary_precision")]
            self.n.parse::<f32>().ok().filter(|float| float.is_finite())
        }
    }

    let number = N { n: 1.5 };
    assert_eq!(number.as_f32(), Some(1.5));
}

#[test]
fn test_as_f32_with_zero_float() {
    struct N {
        n: f64,
    };

    impl N {
        fn as_f32(&self) -> Option<f32> {
            #[cfg(not(feature = "arbitrary_precision"))]
            match self.n {
                n if n.is_sign_positive() => Some(n as f32),
                _ => None,
            }
            #[cfg(feature = "arbitrary_precision")]
            self.n.parse::<f32>().ok().filter(|float| float.is_finite())
        }
    }

    let number = N { n: 0.0 };
    assert_eq!(number.as_f32(), Some(0.0));
}

#[test]
fn test_as_f32_with_negative_float() {
    struct N {
        n: f64,
    };

    impl N {
        fn as_f32(&self) -> Option<f32> {
            #[cfg(not(feature = "arbitrary_precision"))]
            match self.n {
                n if n.is_sign_positive() => Some(n as f32),
                _ => None,
            }
            #[cfg(feature = "arbitrary_precision")]
            self.n.parse::<f32>().ok().filter(|float| float.is_finite())
        }
    }

    let number = N { n: -1.5 };
    assert_eq!(number.as_f32(), None);
}


// Answer 0

#[test]
fn test_as_f32_neg_int() {
    struct N {
        n: NegInt,
    }

    struct NegInt(i32);

    impl N {
        pub(crate) fn as_f32(&self) -> Option<f32> {
            #[cfg(not(feature = "arbitrary_precision"))]
            match &self.n {
                NegInt(n) => Some(*n as f32),
            }
            #[cfg(feature = "arbitrary_precision")]
            self.n.0.parse::<f32>().ok().filter(|float| float.is_finite())
        }
    }

    let neg_int = NegInt(-42);
    let instance = N { n: neg_int };
    assert_eq!(instance.as_f32(), Some(-42.0));

    let neg_int_zero = NegInt(0);
    let instance_zero = N { n: neg_int_zero };
    assert_eq!(instance_zero.as_f32(), Some(0.0));

    let neg_int_large = NegInt(-123456);
    let instance_large = N { n: neg_int_large };
    assert_eq!(instance_large.as_f32(), Some(-123456.0));
}

#[test]
fn test_as_f32_float() {
    struct N {
        n: Float,
    }

    struct Float(f32);

    impl N {
        pub(crate) fn as_f32(&self) -> Option<f32> {
            #[cfg(not(feature = "arbitrary_precision"))]
            match &self.n {
                Float(n) => Some(*n),
            }
            #[cfg(feature = "arbitrary_precision")]
            self.n.0.parse::<f32>().ok().filter(|float| float.is_finite())
        }
    }

    let float_value = Float(-3.14);
    let instance = N { n: float_value };
    assert_eq!(instance.as_f32(), Some(-3.14));
    
    let float_zero = Float(0.0);
    let instance_zero = N { n: float_zero };
    assert_eq!(instance_zero.as_f32(), Some(0.0));

    let float_large = Float(1e38);
    let instance_large = N { n: float_large };
    assert_eq!(instance_large.as_f32(), Some(1e38));
}


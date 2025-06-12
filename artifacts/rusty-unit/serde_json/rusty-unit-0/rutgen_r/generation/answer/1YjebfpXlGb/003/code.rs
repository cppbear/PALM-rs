// Answer 0

#[cfg(test)]
fn test_as_f32_pos_int() {
    struct N {
        n: Box<NEnum>,
    }

    enum NEnum {
        PosInt(u32),
        NegInt(i32),
        Float(f64),
    }

    impl N {
        fn as_f32(&self) -> Option<f32> {
            #[cfg(not(feature = "arbitrary_precision"))]
            match *self.n {
                NEnum::PosInt(n) => Some(n as f32),
                NEnum::NegInt(n) => Some(n as f32),
                NEnum::Float(n) => Some(n as f32),
            }
            #[cfg(feature = "arbitrary_precision")]
            self.n.parse::<f32>().ok().filter(|float| float.is_finite())
        }
    }

    let positive_int = N {
        n: Box::new(NEnum::PosInt(42)),
    };
    assert_eq!(positive_int.as_f32(), Some(42.0));

    let positive_int_max = N {
        n: Box::new(NEnum::PosInt(u32::MAX)),
    };
    assert_eq!(positive_int_max.as_f32(), Some(u32::MAX as f32));
}

#[cfg(test)]
fn test_as_f32_neg_int() {
    struct N {
        n: Box<NEnum>,
    }

    enum NEnum {
        PosInt(u32),
        NegInt(i32),
        Float(f64),
    }

    impl N {
        fn as_f32(&self) -> Option<f32> {
            #[cfg(not(feature = "arbitrary_precision"))]
            match *self.n {
                NEnum::PosInt(n) => Some(n as f32),
                NEnum::NegInt(n) => Some(n as f32),
                NEnum::Float(n) => Some(n as f32),
            }
            #[cfg(feature = "arbitrary_precision")]
            self.n.parse::<f32>().ok().filter(|float| float.is_finite())
        }
    }

    let negative_int = N {
        n: Box::new(NEnum::NegInt(-42)),
    };
    assert_eq!(negative_int.as_f32(), Some(-42.0));

    let negative_int_min = N {
        n: Box::new(NEnum::NegInt(i32::MIN)),
    };
    assert_eq!(negative_int_min.as_f32(), Some(i32::MIN as f32));
}

#[cfg(test)]
fn test_as_f32_float() {
    struct N {
        n: Box<NEnum>,
    }

    enum NEnum {
        PosInt(u32),
        NegInt(i32),
        Float(f64),
    }

    impl N {
        fn as_f32(&self) -> Option<f32> {
            #[cfg(not(feature = "arbitrary_precision"))]
            match *self.n {
                NEnum::PosInt(n) => Some(n as f32),
                NEnum::NegInt(n) => Some(n as f32),
                NEnum::Float(n) => Some(n as f32),
            }
            #[cfg(feature = "arbitrary_precision")]
            self.n.parse::<f32>().ok().filter(|float| float.is_finite())
        }
    }

    let float_value = N {
        n: Box::new(NEnum::Float(3.14)),
    };
    assert_eq!(float_value.as_f32(), Some(3.14_f32));
}


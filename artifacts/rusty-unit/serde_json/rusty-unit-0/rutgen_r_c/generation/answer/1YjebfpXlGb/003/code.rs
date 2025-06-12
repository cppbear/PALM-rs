// Answer 0

#[test]
fn test_as_f32_pos_int() {
    struct NPosInt {
        n: N,
    }

    impl NPosInt {
        pub fn new(value: u64) -> Self {
            NPosInt {
                n: N::PosInt(value),
            }
        }

        pub fn as_f32(&self) -> Option<f32> {
            match self.n {
                N::PosInt(n) => Some(n as f32),
                N::NegInt(n) => Some(n as f32),
                N::Float(n) => Some(n as f32),
            }
        }
    }

    let number = NPosInt::new(42);
    assert_eq!(number.as_f32(), Some(42.0));
}

#[test]
fn test_as_f32_neg_int() {
    struct NNegInt {
        n: N,
    }

    impl NNegInt {
        pub fn new(value: i64) -> Self {
            NNegInt {
                n: N::NegInt(value),
            }
        }

        pub fn as_f32(&self) -> Option<f32> {
            match self.n {
                N::PosInt(n) => Some(n as f32),
                N::NegInt(n) => Some(n as f32),
                N::Float(n) => Some(n as f32),
            }
        }
    }

    let number = NNegInt::new(-42);
    assert_eq!(number.as_f32(), Some(-42.0));
}

#[test]
fn test_as_f32_float() {
    struct NFloat {
        n: N,
    }

    impl NFloat {
        pub fn new(value: f64) -> Self {
            NFloat {
                n: N::Float(value),
            }
        }

        pub fn as_f32(&self) -> Option<f32> {
            match self.n {
                N::PosInt(n) => Some(n as f32),
                N::NegInt(n) => Some(n as f32),
                N::Float(n) => Some(n as f32),
            }
        }
    }

    let number = NFloat::new(42.0);
    assert_eq!(number.as_f32(), Some(42.0));
}


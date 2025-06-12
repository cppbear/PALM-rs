// Answer 0

#[test]
fn test_is_i64_pos_int_within_range() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    impl Number {
        pub fn is_i64(&self) -> bool {
            #[cfg(not(feature = "arbitrary_precision"))]
            match self.n {
                N::PosInt(v) => v <= i64::MAX as u64,
                N::NegInt(_) => true,
                N::Float(_) => false,
            }
            #[cfg(feature = "arbitrary_precision")]
            self.as_i64().is_some()
        }

        #[cfg(feature = "arbitrary_precision")]
        fn as_i64(&self) -> Option<i64> {
            // Implementation is not relevant for the purpose of this test.
            None
        }
    }

    let number = Number { n: N::PosInt(i64::MAX as u64) };
    assert!(number.is_i64());
}

#[test]
fn test_is_i64_pos_int_over_max() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    impl Number {
        pub fn is_i64(&self) -> bool {
            #[cfg(not(feature = "arbitrary_precision"))]
            match self.n {
                N::PosInt(v) => v <= i64::MAX as u64,
                N::NegInt(_) => true,
                N::Float(_) => false,
            }
            #[cfg(feature = "arbitrary_precision")]
            self.as_i64().is_some()
        }

        #[cfg(feature = "arbitrary_precision")]
        fn as_i64(&self) -> Option<i64> {
            // Implementation is not relevant for the purpose of this test.
            None
        }
    }

    let number = Number { n: N::PosInt(i64::MAX as u64 + 1) };
    assert!(!number.is_i64());
}

#[test]
fn test_is_i64_neg_int() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    impl Number {
        pub fn is_i64(&self) -> bool {
            #[cfg(not(feature = "arbitrary_precision"))]
            match self.n {
                N::PosInt(v) => v <= i64::MAX as u64,
                N::NegInt(_) => true,
                N::Float(_) => false,
            }
            #[cfg(feature = "arbitrary_precision")]
            self.as_i64().is_some()
        }

        #[cfg(feature = "arbitrary_precision")]
        fn as_i64(&self) -> Option<i64> {
            // Implementation is not relevant for the purpose of this test.
            None
        }
    }

    let number = Number { n: N::NegInt(-1) };
    assert!(number.is_i64());
}

#[test]
fn test_is_i64_float() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    impl Number {
        pub fn is_i64(&self) -> bool {
            #[cfg(not(feature = "arbitrary_precision"))]
            match self.n {
                N::PosInt(v) => v <= i64::MAX as u64,
                N::NegInt(_) => true,
                N::Float(_) => false,
            }
            #[cfg(feature = "arbitrary_precision")]
            self.as_i64().is_some()
        }

        #[cfg(feature = "arbitrary_precision")]
        fn as_i64(&self) -> Option<i64> {
            // Implementation is not relevant for the purpose of this test.
            None
        }
    }

    let number = Number { n: N::Float(3.14) };
    assert!(!number.is_i64());
}


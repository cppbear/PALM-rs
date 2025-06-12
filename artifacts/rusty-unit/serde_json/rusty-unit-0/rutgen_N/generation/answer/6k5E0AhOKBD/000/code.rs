// Answer 0

#[test]
fn test_is_u64_pos_int() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    impl Number {
        #[cfg(not(feature = "arbitrary_precision"))]
        pub fn is_u64(&self) -> bool {
            match self.n {
                N::PosInt(_) => true,
                N::NegInt(_) | N::Float(_) => false,
            }
        }

        #[cfg(feature = "arbitrary_precision")]
        pub fn is_u64(&self) -> bool {
            self.as_u64().is_some()
        }

        #[cfg(feature = "arbitrary_precision")]
        pub fn as_u64(&self) -> Option<u64> {
            if let N::PosInt(value) = self.n {
                Some(value)
            } else {
                None
            }
        }
    }

    let number = Number { n: N::PosInt(42) };
    assert!(number.is_u64());
}

#[test]
fn test_is_u64_neg_int() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    impl Number {
        #[cfg(not(feature = "arbitrary_precision"))]
        pub fn is_u64(&self) -> bool {
            match self.n {
                N::PosInt(_) => true,
                N::NegInt(_) | N::Float(_) => false,
            }
        }

        #[cfg(feature = "arbitrary_precision")]
        pub fn is_u64(&self) -> bool {
            self.as_u64().is_some()
        }

        #[cfg(feature = "arbitrary_precision")]
        pub fn as_u64(&self) -> Option<u64> {
            if let N::PosInt(value) = self.n {
                Some(value)
            } else {
                None
            }
        }
    }

    let number = Number { n: N::NegInt(-1) };
    assert!(!number.is_u64());
}

#[test]
fn test_is_u64_float() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    impl Number {
        #[cfg(not(feature = "arbitrary_precision"))]
        pub fn is_u64(&self) -> bool {
            match self.n {
                N::PosInt(_) => true,
                N::NegInt(_) | N::Float(_) => false,
            }
        }

        #[cfg(feature = "arbitrary_precision")]
        pub fn is_u64(&self) -> bool {
            self.as_u64().is_some()
        }

        #[cfg(feature = "arbitrary_precision")]
        pub fn as_u64(&self) -> Option<u64> {
            if let N::PosInt(value) = self.n {
                Some(value)
            } else {
                None
            }
        }
    }

    let number = Number { n: N::Float(3.14) };
    assert!(!number.is_u64());
}


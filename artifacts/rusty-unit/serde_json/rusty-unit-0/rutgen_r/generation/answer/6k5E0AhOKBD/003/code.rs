// Answer 0

#[test]
fn test_is_u64_with_pos_int() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    impl Number {
        pub fn is_u64(&self) -> bool {
            #[cfg(not(feature = "arbitrary_precision"))]
            match self.n {
                N::PosInt(_) => true,
                N::NegInt(_) | N::Float(_) => false,
            }
            #[cfg(feature = "arbitrary_precision")]
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

    let pos_int_number = Number { n: N::PosInt(42) };
    assert!(pos_int_number.is_u64());
}

#[test]
fn test_is_u64_with_zero_pos_int() {
    struct Number {
        n: N,
    }

    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    impl Number {
        pub fn is_u64(&self) -> bool {
            #[cfg(not(feature = "arbitrary_precision"))]
            match self.n {
                N::PosInt(_) => true,
                N::NegInt(_) | N::Float(_) => false,
            }
            #[cfg(feature = "arbitrary_precision")]
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

    let zero_pos_int_number = Number { n: N::PosInt(0) };
    assert!(zero_pos_int_number.is_u64());
}


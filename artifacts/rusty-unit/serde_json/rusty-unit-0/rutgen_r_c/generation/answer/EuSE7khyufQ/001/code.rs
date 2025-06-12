// Answer 0

#[test]
fn test_unexpected_float() {
    struct Number {
        n: N,
    }

    #[cfg(not(feature = "arbitrary_precision"))]
    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    impl Number {
        #[cfg(feature = "arbitrary_precision")]
        pub(crate) fn unexpected(&self) -> Unexpected {}

        pub(crate) fn unexpected(&self) -> Unexpected {
            match self.n {
                N::PosInt(u) => Unexpected::Unsigned(u),
                N::NegInt(i) => Unexpected::Signed(i),
                N::Float(f) => Unexpected::Float(f),
            }
        }
    }

    let number = Number { n: N::Float(3.14) };
    let result = number.unexpected();
    
    if let Unexpected::Float(f) = result {
        assert_eq!(f, 3.14);
    } else {
        panic!("Expected Unexpected::Float variant");
    }
}

#[test]
fn test_unexpected_float_negative() {
    struct Number {
        n: N,
    }

    #[cfg(not(feature = "arbitrary_precision"))]
    enum N {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    impl Number {
        #[cfg(feature = "arbitrary_precision")]
        pub(crate) fn unexpected(&self) -> Unexpected {}

        pub(crate) fn unexpected(&self) -> Unexpected {
            match self.n {
                N::PosInt(u) => Unexpected::Unsigned(u),
                N::NegInt(i) => Unexpected::Signed(i),
                N::Float(f) => Unexpected::Float(f),
            }
        }
    }

    let number = Number { n: N::Float(-1.618) };
    let result = number.unexpected();
    
    if let Unexpected::Float(f) = result {
        assert_eq!(f, -1.618);
    } else {
        panic!("Expected Unexpected::Float variant");
    }
}


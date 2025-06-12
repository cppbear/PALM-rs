// Answer 0

#[test]
fn test_as_i128_positive_integer() {
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
        pub fn as_i128(&self) -> Option<i128> {
            #[cfg(not(feature = "arbitrary_precision"))]
            match self.n {
                N::PosInt(n) => Some(n as i128),
                N::NegInt(n) => Some(n as i128),
                N::Float(_) => None,
            }
            #[cfg(feature = "arbitrary_precision")]
            self.n.parse().ok()
        }
    }

    let number = Number { n: N::PosInt(42) };
    assert_eq!(number.as_i128(), Some(42));

    let number_large = Number { n: N::PosInt(127) };
    assert_eq!(number_large.as_i128(), Some(127));
}

#[test]
fn test_as_i128_negative_integer() {
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
        pub fn as_i128(&self) -> Option<i128> {
            #[cfg(not(feature = "arbitrary_precision"))]
            match self.n {
                N::PosInt(n) => Some(n as i128),
                N::NegInt(n) => Some(n as i128),
                N::Float(_) => None,
            }
            #[cfg(feature = "arbitrary_precision")]
            self.n.parse().ok()
        }
    }

    let number = Number { n: N::NegInt(-42) };
    assert_eq!(number.as_i128(), Some(-42));
}

#[test]
fn test_as_i128_float() {
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
        pub fn as_i128(&self) -> Option<i128> {
            #[cfg(not(feature = "arbitrary_precision"))]
            match self.n {
                N::PosInt(n) => Some(n as i128),
                N::NegInt(n) => Some(n as i128),
                N::Float(_) => None,
            }
            #[cfg(feature = "arbitrary_precision")]
            self.n.parse().ok()
        }
    }

    let number_float = Number { n: N::Float(3.14) };
    assert_eq!(number_float.as_i128(), None);
}


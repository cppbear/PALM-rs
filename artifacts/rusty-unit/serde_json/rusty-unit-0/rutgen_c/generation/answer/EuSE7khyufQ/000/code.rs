// Answer 0

#[test]
fn test_unexpected_pos_int() {
    struct NumberPos {
        n: N,
    }

    impl NumberPos {
        pub(crate) fn unexpected(&self) -> Unexpected {
            match self.n {
                N::PosInt(u) => Unexpected::Unsigned(u),
                N::NegInt(i) => Unexpected::Signed(i),
                N::Float(f) => Unexpected::Float(f),
            }
        }
    }

    let num = NumberPos { n: N::PosInt(42) };
    let unexpected = num.unexpected();
    match unexpected {
        Unexpected::Unsigned(value) => assert_eq!(value, 42),
        _ => panic!("Unexpected type"),
    }
}

#[test]
fn test_unexpected_neg_int() {
    struct NumberNeg {
        n: N,
    }

    impl NumberNeg {
        pub(crate) fn unexpected(&self) -> Unexpected {
            match self.n {
                N::PosInt(u) => Unexpected::Unsigned(u),
                N::NegInt(i) => Unexpected::Signed(i),
                N::Float(f) => Unexpected::Float(f),
            }
        }
    }

    let num = NumberNeg { n: N::NegInt(-42) };
    let unexpected = num.unexpected();
    match unexpected {
        Unexpected::Signed(value) => assert_eq!(value, -42),
        _ => panic!("Unexpected type"),
    }
}

#[test]
fn test_unexpected_float() {
    struct NumberFloat {
        n: N,
    }

    impl NumberFloat {
        pub(crate) fn unexpected(&self) -> Unexpected {
            match self.n {
                N::PosInt(u) => Unexpected::Unsigned(u),
                N::NegInt(i) => Unexpected::Signed(i),
                N::Float(f) => Unexpected::Float(f),
            }
        }
    }

    let num = NumberFloat { n: N::Float(3.14) };
    let unexpected = num.unexpected();
    match unexpected {
        Unexpected::Float(value) => assert_eq!(value, 3.14),
        _ => panic!("Unexpected type"),
    }
}


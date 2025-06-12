// Answer 0

#[test]
fn test_unexpected_pos_int() {
    struct N {
        n: NValue,
    }

    enum NValue {
        PosInt(u32),
        NegInt(i32),
        Float(f32),
    }

    enum Unexpected {
        Unsigned(u32),
        Signed(i32),
        Float(f32),
    }

    impl N {
        pub(crate) fn unexpected(&self) -> Unexpected {
            match &self.n {
                NValue::PosInt(u) => Unexpected::Unsigned(*u),
                NValue::NegInt(i) => Unexpected::Signed(*i),
                NValue::Float(f) => Unexpected::Float(*f),
            }
        }
    }

    let positive_integer = N { n: NValue::PosInt(42) };
    match positive_integer.unexpected() {
        Unexpected::Unsigned(value) => assert_eq!(value, 42),
        _ => panic!("Expected an Unsigned value"),
    }
}

#[test]
fn test_unexpected_pos_int_zero() {
    struct N {
        n: NValue,
    }

    enum NValue {
        PosInt(u32),
        NegInt(i32),
        Float(f32),
    }

    enum Unexpected {
        Unsigned(u32),
        Signed(i32),
        Float(f32),
    }

    impl N {
        pub(crate) fn unexpected(&self) -> Unexpected {
            match &self.n {
                NValue::PosInt(u) => Unexpected::Unsigned(*u),
                NValue::NegInt(i) => Unexpected::Signed(*i),
                NValue::Float(f) => Unexpected::Float(*f),
            }
        }
    }

    let positive_integer_zero = N { n: NValue::PosInt(0) };
    match positive_integer_zero.unexpected() {
        Unexpected::Unsigned(value) => assert_eq!(value, 0),
        _ => panic!("Expected an Unsigned value"),
    }
}


// Answer 0

#[test]
fn test_unexpected_float() {
    struct N {
        n: NumType,
    }

    enum NumType {
        PosInt(u64),
        NegInt(i64),
        Float(f64),
    }

    enum Unexpected {
        Unsigned(u64),
        Signed(i64),
        Float(f64),
    }

    impl N {
        pub(crate) fn unexpected(&self) -> Unexpected {
            match self.n {
                NumType::PosInt(u) => Unexpected::Unsigned(u),
                NumType::NegInt(i) => Unexpected::Signed(i),
                NumType::Float(f) => Unexpected::Float(f),
            }
        }
    }

    let float_value: f64 = 3.14;
    let n_instance = N {
        n: NumType::Float(float_value),
    };

    if let Unexpected::Float(result) = n_instance.unexpected() {
        assert_eq!(result, float_value);
    } else {
        panic!("Expected Unexpected::Float but got a different variant");
    }

    let negative_float_value: f64 = -2.71;
    let n_instance_neg = N {
        n: NumType::Float(negative_float_value),
    };

    if let Unexpected::Float(result) = n_instance_neg.unexpected() {
        assert_eq!(result, negative_float_value);
    } else {
        panic!("Expected Unexpected::Float but got a different variant");
    }
}


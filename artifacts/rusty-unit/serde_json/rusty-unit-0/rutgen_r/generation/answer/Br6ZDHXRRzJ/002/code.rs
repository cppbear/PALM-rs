// Answer 0

#[test]
fn test_unexpected_with_array() {
    struct Value {
        data: Vec<ValueType>,
    }

    enum ValueType {
        Null,
        Bool(bool),
        Number(i32),
        String(String),
        Array(Vec<ValueType>),
        Object,
    }

    struct Unexpected;

    impl Unexpected {
        const Unit: Unexpected = Unexpected;
        const Seq: Unexpected = Unexpected;
    }

    fn unexpected(value: &Value) -> Unexpected {
        match value {
            Value { data: _ } if value.matches_array() => Unexpected::Seq,
            _ => Unexpected::Unit, // Default case, not expected to be hit
        }
    }

    impl Value {
        fn matches_array(&self) -> bool {
            if let ValueType::Array(_) = &self.data { 
                true 
            } else { 
                false 
            }
        }
    }

    let array_value = Value {
        data: vec![ValueType::Null, ValueType::Bool(true)],
    };

    let result = unexpected(&array_value);
    assert_eq!(result, Unexpected::Seq);
}


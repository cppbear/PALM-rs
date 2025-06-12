// Answer 0

fn unexpected_test_array() {
    struct Value {
        data: Vec<i32>,
    }

    impl Value {
        fn unexpected(&self) -> Unexpected {
            match self {
                Value { data } if data.is_empty() => Unexpected::Seq, // consider an empty array as valid
                Value { data } => Unexpected::Seq,
            }
        }
    }

    enum Unexpected {
        Unit,
        Bool(bool),
        Str(&'static str),
        Seq,
        Map,
    }

    let value = Value { data: Vec::new() };
    assert_eq!(value.unexpected(), Unexpected::Seq);

    let value_non_empty = Value { data: vec![1, 2, 3] };
    assert_eq!(value_non_empty.unexpected(), Unexpected::Seq);
}


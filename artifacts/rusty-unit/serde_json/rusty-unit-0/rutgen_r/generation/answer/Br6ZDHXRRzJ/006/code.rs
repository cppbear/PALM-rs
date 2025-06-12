// Answer 0

#[test]
fn test_unexpected_value_null() {
    struct Value {
        kind: Kind,
    }

    enum Kind {
        Null,
        Bool(bool),
        Number(i32),
        String(String),
        Array(Vec<Value>),
        Object(std::collections::HashMap<String, Value>),
    }

    enum Unexpected {
        Unit,
        Bool(bool),
        Str(String),
        Seq,
        Map,
    }

    impl Value {
        fn unexpected(&self) -> Unexpected {
            match &self.kind {
                Kind::Null => Unexpected::Unit,
                Kind::Bool(b) => Unexpected::Bool(*b),
                Kind::Number(n) => panic!("Number type not handled"),
                Kind::String(s) => Unexpected::Str(s.clone()),
                Kind::Array(_) => Unexpected::Seq,
                Kind::Object(_) => Unexpected::Map,
            }
        }
    }

    let value = Value { kind: Kind::Null };
    let result = value.unexpected();
    match result {
        Unexpected::Unit => (),
        _ => panic!("Expected Unexpected::Unit, got {:?}", result),
    }
}


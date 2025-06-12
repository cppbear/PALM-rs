// Answer 0

#[test]
fn test_unexpected_string() {
    struct Value {
        kind: ValueKind,
    }

    enum ValueKind {
        Null,
        Bool(bool),
        Number(f64),
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
                ValueKind::Null => Unexpected::Unit,
                ValueKind::Bool(b) => Unexpected::Bool(*b),
                ValueKind::Number(_) => panic!("Not applicable for this test."),
                ValueKind::String(s) => Unexpected::Str(s.clone()),
                ValueKind::Array(_) => Unexpected::Seq,
                ValueKind::Object(_) => Unexpected::Map,
            }
        }
    }

    let value = Value {
        kind: ValueKind::String("test_string".to_string()),
    };
    
    if let Unexpected::Str(s) = value.unexpected() {
        assert_eq!(s, "test_string");
    } else {
        panic!("Expected Unexpected::Str but got a different variant.");
    }
}


// Answer 0

#[test]
fn test_unexpected_null() {
    struct Value {
        kind: ValueKind,
    }

    enum ValueKind {
        Null,
        // other variants omitted for brevity
    }

    enum Unexpected {
        Unit,
        Bool(bool),
        Str(String),
        Seq,
        Map,
        // other variants omitted for brevity
    }

    impl Value {
        fn unexpected(&self) -> Unexpected {
            match self.kind {
                ValueKind::Null => Unexpected::Unit,
                // other cases omitted for brevity
            }
        }
    }

    let value = Value { kind: ValueKind::Null };
    let result = value.unexpected();
    
    match result {
        Unexpected::Unit => (),
        _ => panic!("Expected Unexpected::Unit, got {:?}", result),
    }
}


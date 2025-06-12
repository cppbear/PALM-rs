// Answer 0

#[test]
fn test_is_boolean_true() {
    struct Value {
        kind: ValueKind,
    }

    enum ValueKind {
        Bool(bool),
        String(String),
        // other variants can be added if needed
    }

    impl Value {
        fn as_bool(&self) -> Option<bool> {
            if let ValueKind::Bool(b) = &self.kind {
                Some(*b)
            } else {
                None
            }
        }

        fn is_boolean(&self) -> bool {
            self.as_bool().is_some()
        }
    }

    let boolean_value = Value { kind: ValueKind::Bool(true) };
    assert!(boolean_value.is_boolean());
}

#[test]
fn test_is_boolean_false() {
    struct Value {
        kind: ValueKind,
    }

    enum ValueKind {
        Bool(bool),
        String(String),
        // other variants can be added if needed
    }

    impl Value {
        fn as_bool(&self) -> Option<bool> {
            if let ValueKind::Bool(b) = &self.kind {
                Some(*b)
            } else {
                None
            }
        }

        fn is_boolean(&self) -> bool {
            self.as_bool().is_some()
        }
    }

    let string_value = Value { kind: ValueKind::String("false".to_string()) };
    assert!(!string_value.is_boolean());
}


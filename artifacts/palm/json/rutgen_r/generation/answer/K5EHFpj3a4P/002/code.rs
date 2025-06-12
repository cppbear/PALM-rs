// Answer 0

#[test]
fn test_as_bool_true() {
    struct Value {
        value: bool,
    }
    
    impl Value {
        fn as_bool(&self) -> Option<bool> {
            match self {
                Value { value } => Some(*value),
            }
        }
    }

    let v_true = Value { value: true };
    assert_eq!(v_true.as_bool(), Some(true));
}

#[test]
fn test_as_bool_false() {
    struct Value {
        value: bool,
    }
    
    impl Value {
        fn as_bool(&self) -> Option<bool> {
            match self {
                Value { value } => Some(*value),
            }
        }
    }

    let v_false = Value { value: false };
    assert_eq!(v_false.as_bool(), Some(false));
}


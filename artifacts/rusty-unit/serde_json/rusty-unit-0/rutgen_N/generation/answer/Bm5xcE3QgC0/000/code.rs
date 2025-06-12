// Answer 0

#[test]
fn test_eq_bool_true() {
    struct Value {
        boolean: Option<bool>,
    }

    impl Value {
        fn as_bool(&self) -> Option<bool> {
            self.boolean
        }
    }

    let value_true = Value { boolean: Some(true) };
    let result = eq_bool(&value_true, true);
    assert!(result);
}

#[test]
fn test_eq_bool_false() {
    struct Value {
        boolean: Option<bool>,
    }

    impl Value {
        fn as_bool(&self) -> Option<bool> {
            self.boolean
        }
    }

    let value_false = Value { boolean: Some(false) };
    let result = eq_bool(&value_false, false);
    assert!(result);
}

#[test]
fn test_eq_bool_none() {
    struct Value {
        boolean: Option<bool>,
    }

    impl Value {
        fn as_bool(&self) -> Option<bool> {
            self.boolean
        }
    }

    let value_none = Value { boolean: None };
    let result_true = eq_bool(&value_none, true);
    let result_false = eq_bool(&value_none, false);
    assert!(!result_true);
    assert!(!result_false);
}


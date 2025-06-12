// Answer 0

#[test]
fn test_is_boolean_true() {
    struct ValueWrapper {
        value: serde_json::Value,
    }

    impl ValueWrapper {
        fn is_boolean(&self) -> bool {
            self.value.as_bool().is_some()
        }
    }

    let v = ValueWrapper { value: serde_json::json!(true) };
    assert!(v.is_boolean());

    let v_false = ValueWrapper { value: serde_json::json!(false) };
    assert!(v_false.is_boolean());
}

#[test]
fn test_is_boolean_false() {
    struct ValueWrapper {
        value: serde_json::Value,
    }

    impl ValueWrapper {
        fn is_boolean(&self) -> bool {
            self.value.as_bool().is_some()
        }
    }
    
    let v_string = ValueWrapper { value: serde_json::json!("false") };
    assert!(!v_string.is_boolean());

    let v_number = ValueWrapper { value: serde_json::json!(0) };
    assert!(!v_number.is_boolean());

    let v_null = ValueWrapper { value: serde_json::json!(null) };
    assert!(!v_null.is_boolean());

    let v_array = ValueWrapper { value: serde_json::json!([true, false]) };
    assert!(!v_array.is_boolean());

    let v_object = ValueWrapper { value: serde_json::json!({"key": "value"}) };
    assert!(!v_object.is_boolean());
}


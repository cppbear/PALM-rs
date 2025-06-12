// Answer 0

fn index_or_insert_test_valid_index() {
    use serde_json::Value;

    let index = 1;
    let mut value = Value::Array(vec![Value::Number(1.into()), Value::Number(2.into()), Value::Number(3.into())]);
    
    let result = index_or_insert(&index, &mut value);
    
    assert_eq!(result, &mut Value::Number(2.into()));
}

fn index_or_insert_test_out_of_bounds() {
    use serde_json::Value;

    let index = 5;
    let mut value = Value::Array(vec![Value::Number(1.into()), Value::Number(2.into()), Value::Number(3.into())]);
    
    let result = std::panic::catch_unwind(|| {
        index_or_insert(&index, &mut value);
    });
    
    assert!(result.is_err());
}

fn index_or_insert_test_negative_index() {
    use serde_json::Value;

    let index = -1;
    let mut value = Value::Array(vec![Value::Number(1.into()), Value::Number(2.into()), Value::Number(3.into())]);
    
    let result = std::panic::catch_unwind(|| {
        index_or_insert(&index, &mut value);
    });
    
    assert!(result.is_err());
}


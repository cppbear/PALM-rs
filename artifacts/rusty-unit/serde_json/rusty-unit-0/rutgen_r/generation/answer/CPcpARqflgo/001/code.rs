// Answer 0

#[test]
fn test_index_or_insert_non_array() {
    use serde_json::json;
    use serde_json::Value;

    let value = Value::Object(Default::default());
    let mut value_ref = value.clone();
    let index: usize = 0; // example index

    // Panic should occur because value is not an Array
    let result = std::panic::catch_unwind(|| {
        let _ = index_or_insert(&index, &mut value_ref);
    });

    assert!(result.is_err(), "Expected panic when accessing index on non-array type");
}

#[test]
fn test_index_or_insert_non_array_with_string() {
    use serde_json::json;
    use serde_json::Value;

    let value = Value::String("foo".to_string());
    let mut value_ref = value.clone();
    let index: usize = 0; // example index

    // Panic should occur because value is not an Array
    let result = std::panic::catch_unwind(|| {
        let _ = index_or_insert(&index, &mut value_ref);
    });

    assert!(result.is_err(), "Expected panic when accessing index on non-array type (string)");
}

#[test]
fn test_index_or_insert_non_array_with_number() {
    use serde_json::json;
    use serde_json::Value;

    let value = Value::Number(serde_json::Number::from(42));
    let mut value_ref = value.clone();
    let index: usize = 0; // example index

    // Panic should occur because value is not an Array
    let result = std::panic::catch_unwind(|| {
        let _ = index_or_insert(&index, &mut value_ref);
    });

    assert!(result.is_err(), "Expected panic when accessing index on non-array type (number)");
}


// Answer 0

#[test]
fn test_index_or_insert_panic_on_non_array() {
    let value = Value::Null;

    // Attempt to call index_or_insert on a Value::Null which is not an array.
    let index = 0;
    let result = std::panic::catch_unwind(|| {
        let _ = &index.index_or_insert(&mut value);
    });

    // Ensure that the panic occurs as expected.
    assert!(result.is_err());
}

#[test]
fn test_index_or_insert_panic_on_object() {
    let value = Value::Object(Map::new());

    // Attempt to call index_or_insert on a Value::Object which is not an array.
    let index = 0;
    let result = std::panic::catch_unwind(|| {
        let _ = &index.index_or_insert(&mut value);
    });

    // Ensure that the panic occurs as expected.
    assert!(result.is_err());
}

#[test]
fn test_index_or_insert_panic_on_string() {
    let value = Value::String(String::from("string"));

    // Attempt to call index_or_insert on a Value::String which is not an array.
    let index = 1;
    let result = std::panic::catch_unwind(|| {
        let _ = &index.index_or_insert(&mut value);
    });

    // Ensure that the panic occurs as expected.
    assert!(result.is_err());
}


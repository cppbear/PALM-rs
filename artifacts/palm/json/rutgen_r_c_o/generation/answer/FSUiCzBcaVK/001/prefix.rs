// Answer 0

#[test]
fn test_to_string_pretty_with_null() {
    let value: Option<&()> = None;
    let _ = to_string_pretty(value);
}

#[test]
fn test_to_string_pretty_with_empty_struct() {
    #[derive(Serialize)]
    struct EmptyStruct;
    let value = EmptyStruct;
    let _ = to_string_pretty(&value);
}

#[test]
fn test_to_string_pretty_with_non_serializable_type() {
    struct NonSerializable;
    let value = NonSerializable;
    let _ = to_string_pretty(&value);
}

#[test]
fn test_to_string_pretty_with_vector_of_non_serializable() {
    let value: Vec<NonSerializable> = vec![NonSerializable, NonSerializable];
    let _ = to_string_pretty(&value);
}

#[test]
fn test_to_string_pretty_with_map_with_non_string_keys() {
    use std::collections::HashMap;
    let mut value: HashMap<i32, String> = HashMap::new();
    value.insert(1, "one".to_string());
    let _ = to_string_pretty(&value);
}

#[test]
fn test_to_string_pretty_with_complex_nested_structure() {
    #[derive(Serialize)]
    struct Nested {
        inner: NonSerializable,
    }
    let value = Nested { inner: NonSerializable };
    let _ = to_string_pretty(&value);
}


// Answer 0

#[test]
fn test_insert_null_value() {
    let mut map = serde_json::Map::new();
    match map.entry("test_null") {
        Entry::Vacant(vacant) => {
            vacant.insert(Value::Null);
        }
        Entry::Occupied(_) => unreachable!(),
    }
}

#[test]
fn test_insert_boolean_value_true() {
    let mut map = serde_json::Map::new();
    match map.entry("test_true") {
        Entry::Vacant(vacant) => {
            vacant.insert(Value::Bool(true));
        }
        Entry::Occupied(_) => unreachable!(),
    }
}

#[test]
fn test_insert_boolean_value_false() {
    let mut map = serde_json::Map::new();
    match map.entry("test_false") {
        Entry::Vacant(vacant) => {
            vacant.insert(Value::Bool(false));
        }
        Entry::Occupied(_) => unreachable!(),
    }
}

#[test]
fn test_insert_number_value() {
    let mut map = serde_json::Map::new();
    match map.entry("test_number") {
        Entry::Vacant(vacant) => {
            vacant.insert(Value::Number(100));
        }
        Entry::Occupied(_) => unreachable!(),
    }
}

#[test]
fn test_insert_string_value() {
    let mut map = serde_json::Map::new();
    match map.entry("test_string") {
        Entry::Vacant(vacant) => {
            vacant.insert(Value::String("Hello, World!".to_string()));
        }
        Entry::Occupied(_) => unreachable!(),
    }
}

#[test]
fn test_insert_array_value() {
    let mut map = serde_json::Map::new();
    let array_value = Value::Array(vec![
        Value::Null,
        Value::Bool(true),
        Value::Number(123),
        Value::String("Test".to_string()),
    ]);
    match map.entry("test_array") {
        Entry::Vacant(vacant) => {
            vacant.insert(array_value);
        }
        Entry::Occupied(_) => unreachable!(),
    }
}

#[test]
fn test_insert_object_value() {
    let mut map = serde_json::Map::new();
    let object_value = Value::Object(serde_json::Map::from_iter(vec![
        ("key1".to_string(), Value::Number(1)),
        ("key2".to_string(), Value::String("value".to_string())),
    ]));
    match map.entry("test_object") {
        Entry::Vacant(vacant) => {
            vacant.insert(object_value);
        }
        Entry::Occupied(_) => unreachable!(),
    }
}

#[test]
fn test_insert_empty_string_key() {
    let mut map = serde_json::Map::new();
    match map.entry("") {
        Entry::Vacant(vacant) => {
            vacant.insert(Value::String("Empty key".to_string()));
        }
        Entry::Occupied(_) => unreachable!(),
    }
}

#[test]
fn test_insert_large_object() {
    let mut map = serde_json::Map::new();
    let mut large_object = serde_json::Map::new();
    for i in 0..100 {
        large_object.insert(format!("key{}", i), Value::Number(i));
    }
    match map.entry("large_object") {
        Entry::Vacant(vacant) => {
            vacant.insert(Value::Object(large_object));
        }
        Entry::Occupied(_) => unreachable!(),
    }
}


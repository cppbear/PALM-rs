// Answer 0

#[test]
fn test_or_insert_with_string_value() {
    let mut map = serde_json::Map::new();
    let entry = Entry::Vacant(VacantEntry {
        vacant: map.entry("key_string"),
    });
    entry.or_insert_with(|| Value::String(String::from("test")));
}

#[test]
fn test_or_insert_with_number_value() {
    let mut map = serde_json::Map::new();
    let entry = Entry::Vacant(VacantEntry {
        vacant: map.entry("key_number"),
    });
    entry.or_insert_with(|| Value::Number(Number::from(42)));
}

#[test]
fn test_or_insert_with_bool_value() {
    let mut map = serde_json::Map::new();
    let entry = Entry::Vacant(VacantEntry {
        vacant: map.entry("key_bool"),
    });
    entry.or_insert_with(|| Value::Bool(true));
}

#[test]
fn test_or_insert_with_null_value() {
    let mut map = serde_json::Map::new();
    let entry = Entry::Vacant(VacantEntry {
        vacant: map.entry("key_null"),
    });
    entry.or_insert_with(|| Value::Null);
}

#[test]
fn test_or_insert_with_array_value() {
    let mut map = serde_json::Map::new();
    let entry = Entry::Vacant(VacantEntry {
        vacant: map.entry("key_array"),
    });
    entry.or_insert_with(|| Value::Array(vec![Value::String(String::from("item1")), Value::Null]));
}

#[test]
fn test_or_insert_with_object_value() {
    let mut map = serde_json::Map::new();
    let entry = Entry::Vacant(VacantEntry {
        vacant: map.entry("key_object"),
    });
    entry.or_insert_with(|| Value::Object(Map::new()));
}


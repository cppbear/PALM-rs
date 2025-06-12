// Answer 0

#[test]
fn test_index_into_existing_key() {
    let key = String::from("valid_key");
    let value = Value::Object(Map::from([(String::from("valid_key"), Value::Bool(true))]));
    key.index_into(&value);
}

#[test]
fn test_index_into_non_existing_key() {
    let key = String::from("non_existing_key");
    let value = Value::Object(Map::from([(String::from("some_key"), Value::Bool(false))]));
    key.index_into(&value);
}

#[test]
fn test_index_into_empty_string() {
    let key = String::from("");
    let value = Value::Object(Map::from([(String::from("some_key"), Value::Bool(false))]));
    key.index_into(&value);
}

#[test]
fn test_index_into_special_characters() {
    let key = String::from("key_with_special_characters!@#");
    let value = Value::Object(Map::from([(String::from("key_with_special_characters!@#"), Value::String(String::from("value")))]));
    key.index_into(&value);
}

#[test]
fn test_index_into_numeric_key() {
    let key = String::from("123key");
    let value = Value::Object(Map::from([(String::from("123key"), Value::Number(Number::from(1)))]));
    key.index_into(&value);
}

#[test]
fn test_index_into_mixed_case_key() {
    let key = String::from("MixedCaseKey");
    let value = Value::Object(Map::from([(String::from("MixedCaseKey"), Value::String(String::from("test")))]));
    key.index_into(&value);
}


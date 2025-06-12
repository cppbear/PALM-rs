// Answer 0

#[test]
fn test_empty_string() {
    let deserializer = Deserializer::from_str("");
}

#[test]
fn test_space_string() {
    let deserializer = Deserializer::from_str(" ");
}

#[test]
fn test_simple_json_object() {
    let deserializer = Deserializer::from_str("{\"key\":\"value\"}");
}

#[test]
fn test_simple_json_number() {
    let deserializer = Deserializer::from_str("{\"key\":123}");
}

#[test]
fn test_json_array() {
    let deserializer = Deserializer::from_str("[1, 2, 3]");
}

#[test]
fn test_nested_json_object() {
    let deserializer = Deserializer::from_str("{\"key\":{\"nested_key\":\"nested_value\"}}");
}

#[test]
fn test_empty_json_object() {
    let deserializer = Deserializer::from_str("{ }");
}

#[test]
fn test_empty_json_array() {
    let deserializer = Deserializer::from_str("[ ]");
}

#[test]
fn test_json_with_array_in_object() {
    let deserializer = Deserializer::from_str("{\"key\":[{\"sub_key\":\"sub_value\"}]}");
}

#[test]
fn test_json_with_null() {
    let deserializer = Deserializer::from_str("{\"key\":null}");
}


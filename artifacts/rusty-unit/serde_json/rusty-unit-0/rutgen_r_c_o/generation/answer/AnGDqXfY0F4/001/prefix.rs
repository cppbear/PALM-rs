// Answer 0

#[test]
fn test_valid_number_integer() {
    let input = "123";
    let _ = from_str(input);
}

#[test]
fn test_valid_number_float() {
    let input = "45.67";
    let _ = from_str(input);
}

#[test]
fn test_valid_null() {
    let input = "null";
    let _ = from_str(input);
}

#[test]
fn test_valid_boolean_true() {
    let input = "true";
    let _ = from_str(input);
}

#[test]
fn test_valid_boolean_false() {
    let input = "false";
    let _ = from_str(input);
}

#[test]
fn test_valid_string() {
    let input = "\"abc\"";
    let _ = from_str(input);
}

#[test]
fn test_valid_array() {
    let input = "[\"item1\", \"item2\"]";
    let _ = from_str(input);
}

#[test]
fn test_valid_object() {
    let input = "{\"key\": \"value\"}";
    let _ = from_str(input);
}

#[test]
fn test_empty_string() {
    let input = "";
    let _ = from_str(input);
}

#[test]
#[should_panic]
fn test_malformed_json_missing_bracket() {
    let input = "[\"missing bracket\"";
    let _ = from_str(input);
}

#[test]
#[should_panic]
fn test_malformed_json_unquoted_key() {
    let input = "{key: \"value\"}";
    let _ = from_str(input);
}

#[test]
#[should_panic]
fn test_malformed_json_invalid_number() {
    let input = "12.34.56";
    let _ = from_str(input);
}


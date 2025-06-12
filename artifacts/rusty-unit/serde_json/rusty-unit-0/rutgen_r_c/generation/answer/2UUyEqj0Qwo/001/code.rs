// Answer 0

#[test]
fn test_collect_str_with_string() {
    let serializer = Serializer;
    let input = "example string";
    let result = serializer.collect_str(&input);
    assert_eq!(result, Ok(Value::String(input.to_string())));
}

#[test]
fn test_collect_str_with_char() {
    let serializer = Serializer;
    let input = 'a';
    let result = serializer.collect_str(&input);
    assert_eq!(result, Ok(Value::String(input.to_string())));
}

#[test]
fn test_collect_str_with_owned_string() {
    let serializer = Serializer;
    let input = String::from("owned string");
    let result = serializer.collect_str(&input);
    assert_eq!(result, Ok(Value::String(input.to_string())));
}

#[test]
fn test_collect_str_with_empty_string() {
    let serializer = Serializer;
    let input = "";
    let result = serializer.collect_str(&input);
    assert_eq!(result, Ok(Value::String(input.to_string())));
}


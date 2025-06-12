// Answer 0

#[test]
fn test_serialize_str() {
    let value = "Hello, World!";
    let result = serialize_str(value);
    assert_eq!(result, Ok("Hello, World!".to_owned()));
}

#[test]
fn test_serialize_empty_str() {
    let value = "";
    let result = serialize_str(value);
    assert_eq!(result, Ok("".to_owned()));
}

#[test]
fn test_serialize_special_characters() {
    let value = "Hello, 😊🌍!";
    let result = serialize_str(value);
    assert_eq!(result, Ok("Hello, 😊🌍!".to_owned()));
}

#[test]
fn test_serialize_long_string() {
    let value = "a".repeat(1000); // Long string of 1000 'a's
    let result = serialize_str(&value);
    assert_eq!(result, Ok(value.to_owned()));
}

#[test]
fn test_serialize_whitespace() {
    let value = "   ";
    let result = serialize_str(value);
    assert_eq!(result, Ok("   ".to_owned()));
}


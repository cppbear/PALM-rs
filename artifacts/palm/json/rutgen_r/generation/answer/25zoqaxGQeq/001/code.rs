// Answer 0

#[test]
fn test_serialize_char_valid_character() {
    let c: char = 'a';
    let result = serialize_char(c);
    assert_eq!(result, Ok(Value::String("a".to_string())));
}

#[test]
fn test_serialize_char_numeric_character() {
    let c: char = '1';
    let result = serialize_char(c);
    assert_eq!(result, Ok(Value::String("1".to_string())));
}

#[test]
fn test_serialize_char_special_character() {
    let c: char = '!'; 
    let result = serialize_char(c);
    assert_eq!(result, Ok(Value::String("!".to_string())));
}

#[test]
fn test_serialize_char_space_character() {
    let c: char = ' ';
    let result = serialize_char(c);
    assert_eq!(result, Ok(Value::String(" ".to_string())));
}

#[test]
fn test_serialize_char_non_ascii_character() {
    let c: char = 'ñ'; 
    let result = serialize_char(c);
    assert_eq!(result, Ok(Value::String("ñ".to_string())));
}


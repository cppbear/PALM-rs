// Answer 0

#[test]
fn test_serialize_char_valid() {
    let value: char = 'a';
    let result = serialize_char(value);
    assert_eq!(result, Ok("a".to_string()));
}

#[test]
fn test_serialize_char_unicode() {
    let value: char = '😊';
    let result = serialize_char(value);
    assert_eq!(result, Ok("😊".to_string()));
}

#[test]
fn test_serialize_char_newline() {
    let value: char = '\n';
    let result = serialize_char(value);
    assert_eq!(result, Ok("\n".to_string()));
}

#[test]
fn test_serialize_char_tab() {
    let value: char = '\t';
    let result = serialize_char(value);
    assert_eq!(result, Ok("\t".to_string()));
}

#[test]
fn test_serialize_char_empty() {
    // Although in Rust, an empty char is not valid, 
    // we will test the boundary of a non-visible character
    let value: char = '\0';
    let result = serialize_char(value);
    assert_eq!(result, Ok("\0".to_string()));
}


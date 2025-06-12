// Answer 0

#[test]
fn test_serialize_char() {
    let value: char = 'a';
    let result = serialize_char(value);
    assert_eq!(result.unwrap(), "a");
}

#[test]
fn test_serialize_special_char() {
    let value: char = '\n';
    let result = serialize_char(value);
    assert_eq!(result.unwrap(), "\n");
}

#[test]
fn test_serialize_empty_char() {
    #[should_panic]
    fn serialize_empty() {
        let value: char = '\0'; // null character
        serialize_char(value).unwrap();
    }
}


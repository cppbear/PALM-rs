// Answer 0

#[derive(Debug)]
struct StringSink<'a> {
    string: &'a mut String,
}

pub(crate) fn new(s: &mut String) -> StringSink {
    StringSink { string: s }
}

#[test]
fn test_new_with_empty_string() {
    let mut empty_string = String::new();
    let result = new(&mut empty_string);
    assert_eq!(result.string, &mut empty_string);
}

#[test]
fn test_new_with_non_empty_string() {
    let mut hello_string = String::from("Hello");
    let result = new(&mut hello_string);
    assert_eq!(result.string, &mut hello_string);
}

#[test]
fn test_new_with_space_string() {
    let mut space_string = String::from("   ");
    let result = new(&mut space_string);
    assert_eq!(result.string, &mut space_string);
}

#[test]
fn test_new_with_special_characters() {
    let mut special_string = String::from("!@#$%^&*()");
    let result = new(&mut special_string);
    assert_eq!(result.string, &mut special_string);
}

#[test]
fn test_new_with_numeric_string() {
    let mut numeric_string = String::from("123456");
    let result = new(&mut numeric_string);
    assert_eq!(result.string, &mut numeric_string);
}


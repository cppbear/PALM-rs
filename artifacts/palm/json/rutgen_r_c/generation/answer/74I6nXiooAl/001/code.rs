// Answer 0

#[test]
fn test_str_read_new() {
    let input = "Hello, world!";
    let str_read = StrRead::new(input);
    assert_eq!(str_read.delegate.slice, input.as_bytes());
    assert_eq!(str_read.delegate.index, 0);
}

#[test]
fn test_str_read_new_empty_string() {
    let input = "";
    let str_read = StrRead::new(input);
    assert_eq!(str_read.delegate.slice, input.as_bytes());
    assert_eq!(str_read.delegate.index, 0);
}

#[test]
fn test_str_read_new_unicode_string() {
    let input = "你好，世界！";
    let str_read = StrRead::new(input);
    assert_eq!(str_read.delegate.slice, input.as_bytes());
    assert_eq!(str_read.delegate.index, 0);
}

#[test]
fn test_str_read_new_whitespace_string() {
    let input = "   ";
    let str_read = StrRead::new(input);
    assert_eq!(str_read.delegate.slice, input.as_bytes());
    assert_eq!(str_read.delegate.index, 0);
}

#[test]
fn test_str_read_new_special_characters_string() {
    let input = "!@#$%^&*()_+";
    let str_read = StrRead::new(input);
    assert_eq!(str_read.delegate.slice, input.as_bytes());
    assert_eq!(str_read.delegate.index, 0);
}


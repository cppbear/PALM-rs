// Answer 0

#[test]
fn test_collect_str_with_simple_string() {
    let input = "Hello, Rust!".to_string();
    let result = collect_str(&input).unwrap();
    assert_eq!(result, Value::String("Hello, Rust!".to_string()));
}

#[test]
fn test_collect_str_with_number_as_string() {
    let input = 12345;
    let result = collect_str(&input).unwrap();
    assert_eq!(result, Value::String("12345".to_string()));
}

#[test]
fn test_collect_str_with_empty_string() {
    let input = "".to_string();
    let result = collect_str(&input).unwrap();
    assert_eq!(result, Value::String("".to_string()));
}

#[test]
fn test_collect_str_with_special_characters() {
    let input = "!@#$%^&*()".to_string();
    let result = collect_str(&input).unwrap();
    assert_eq!(result, Value::String("!@#$%^&*()".to_string()));
}

#[test]
fn test_collect_str_with_string_containing_spaces() {
    let input = "   Leading and trailing spaces   ".to_string();
    let result = collect_str(&input).unwrap();
    assert_eq!(result, Value::String("   Leading and trailing spaces   ".to_string()));
}

#[test]
fn test_collect_str_with_non_ascii_characters() {
    let input = "こんにちは".to_string();
    let result = collect_str(&input).unwrap();
    assert_eq!(result, Value::String("こんにちは".to_string()));
}


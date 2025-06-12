// Answer 0

#[test]
fn test_collect_str_with_string() {
    let value = "Hello, world!";
    let result = collect_str(value);
    assert_eq!(result, Ok("Hello, world!".to_string()));
}

#[test]
fn test_collect_str_with_integer() {
    let value = 42;
    let result = collect_str(&value);
    assert_eq!(result, Ok("42".to_string()));
}

#[test]
fn test_collect_str_with_float() {
    let value = 3.14;
    let result = collect_str(&value);
    assert_eq!(result, Ok("3.14".to_string()));
}

#[test]
fn test_collect_str_with_empty_string() {
    let value = "";
    let result = collect_str(value);
    assert_eq!(result, Ok("".to_string()));
}

#[test]
fn test_collect_str_with_negative_integer() {
    let value = -10;
    let result = collect_str(&value);
    assert_eq!(result, Ok("-10".to_string()));
}

#[test]
fn test_collect_str_with_large_integer() {
    let value = 1_000_000;
    let result = collect_str(&value);
    assert_eq!(result, Ok("1000000".to_string()));
}

#[test]
fn test_collect_str_with_special_characters() {
    let value = "!@#$%^&*()";
    let result = collect_str(value);
    assert_eq!(result, Ok("!@#$%^&*()".to_string()));
}


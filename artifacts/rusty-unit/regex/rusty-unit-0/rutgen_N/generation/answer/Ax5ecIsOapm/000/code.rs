// Answer 0

#[test]
fn test_repeat_char_zero_count() {
    let result = repeat_char('a', 0);
    assert_eq!(result, "");
}

#[test]
fn test_repeat_char_positive_count() {
    let result = repeat_char('b', 5);
    assert_eq!(result, "bbbbb");
}

#[test]
fn test_repeat_char_large_count() {
    let result = repeat_char('c', 1000);
    assert_eq!(result.len(), 1000);
    assert_eq!(result.chars().next().unwrap(), 'c');
}

#[test]
fn test_repeat_char_non_alphanumeric() {
    let result = repeat_char('!', 3);
    assert_eq!(result, "!!!");
}

#[test]
fn test_repeat_char_character_edge_case() {
    let result = repeat_char('\0', 3);
    assert_eq!(result, "\0\0\0");
}


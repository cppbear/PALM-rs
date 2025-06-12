// Answer 0

#[test]
fn test_repeat_char_zero_count() {
    let result = repeat_char('a', 0);
    assert_eq!(result, "");
}

#[test]
fn test_repeat_char_one_count() {
    let result = repeat_char('b', 1);
    assert_eq!(result, "b");
}

#[test]
fn test_repeat_char_multiple_count() {
    let result = repeat_char('c', 5);
    assert_eq!(result, "ccccc");
}

#[test]
fn test_repeat_char_large_count() {
    let result = repeat_char('d', 1000);
    assert_eq!(result.chars().count(), 1000);
    assert!(result.starts_with("d"));
    assert!(result.ends_with("d"));
}

#[test]
fn test_repeat_char_special_character() {
    let result = repeat_char('!', 3);
    assert_eq!(result, "!!!");
}

#[test]
fn test_repeat_char_empty_character() {
    let result = repeat_char(' ', 4);
    assert_eq!(result, "    ");
}

#[test]
fn test_repeat_char_unicode_character() {
    let result = repeat_char('ä', 2);
    assert_eq!(result, "ää");
}


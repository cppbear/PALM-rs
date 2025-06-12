// Answer 0

#[test]
fn test_char_is_none_returns_true_for_none() {
    let none_char = Char(u32::MAX);
    assert!(none_char.is_none());
}

#[test]
fn test_char_is_none_returns_false_for_valid_character() {
    let valid_char = Char(97); // 'a' in UTF-32
    assert!(!valid_char.is_none());
}


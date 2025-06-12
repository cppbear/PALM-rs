// Answer 0

#[test]
fn test_is_word_character_ascii_alphabetic() {
    assert!(is_word_character('a'));
    assert!(is_word_character('Z'));
    assert!(is_word_character('0'));
}

#[test]
fn test_is_word_character_unicode() {
    assert!(is_word_character('é'));
    assert!(is_word_character('中'));
    assert!(is_word_character('🧑')); // Emoji character
}

#[test]
fn test_is_word_character_non_word() {
    assert!(!is_word_character('!'));
    assert!(!is_word_character(' '));
    assert!(!is_word_character('\n'));
}

#[test]
fn test_is_word_character_connector_punctuation() {
    assert!(is_word_character('_'));
}

#[test]
fn test_is_word_character_boundary() {
    assert!(is_word_character('\u{002D}')); // Hyphen
    assert!(!is_word_character('\u{007F}')); // DEL character
}


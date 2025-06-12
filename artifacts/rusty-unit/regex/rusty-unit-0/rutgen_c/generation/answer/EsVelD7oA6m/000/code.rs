// Answer 0

#[test]
fn test_is_word_character_ascii() {
    assert!(is_word_character('a'));
    assert!(is_word_character('Z'));
    assert!(is_word_character('0'));
    assert!(is_word_character('_'));
    assert!(!is_word_character('!'));
    assert!(!is_word_character(' '));
}

#[test]
fn test_is_word_character_unicode() {
    assert!(is_word_character('é')); // Latin small letter e with acute
    assert!(is_word_character('ß')); // Latin small letter sharp s
    assert!(is_word_character('あ')); // Hiragana letter a
    assert!(is_word_character('中')); // CJK Unified Ideograph
    assert!(!is_word_character('!')); // punctuation
    assert!(!is_word_character(' ')); // space
}

#[test]
fn test_is_word_character_boundary() {
    assert!(!is_word_character('\u{0000}')); // Null character
    assert!(!is_word_character('\u{007F}')); // Delete character
}


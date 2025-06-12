// Answer 0

#[test]
fn test_is_word_character_non_word_byte() {
    let c = '\x7F';
    is_word_character(c);
}

#[test]
fn test_is_word_character_non_word_char_beyond_ascii() {
    let c = 'ñ';
    is_word_character(c);
}

#[test]
fn test_is_word_character_special_characters() {
    let c = '#';
    is_word_character(c);
}

#[test]
fn test_is_word_character_max_unicode() {
    let c = '\u{FFFF}';
    is_word_character(c);
}


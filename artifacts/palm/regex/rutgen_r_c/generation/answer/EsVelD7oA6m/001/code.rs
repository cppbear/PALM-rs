// Answer 0

#[test]
fn test_is_word_character_with_valid_ascii() {
    // Valid ASCII characters that are also word characters
    let valid_chars = vec![
        'a', // Alphabetic
        'Z', // Alphabetic
        '0', // Decimal number
        '_', // Connector punctuation
        '9', // Decimal number
    ];
    
    for c in valid_chars {
        assert_eq!(is_word_character(c), true);
    }
}

#[test]
fn test_is_word_character_with_upper_boundary() {
    // Check the upper boundary for ASCII characters
    let upper_boundary_char = 0x7F as char;
    assert_eq!(is_word_character(upper_boundary_char), false);
}

#[test]
fn test_is_word_character_with_non_word_byte() {
    // Test ASCII characters that are not word characters
    let non_word_chars = vec![
        ' ', // Space
        '!', // Exclamation
        ',', // Comma
        '@', // At symbol
        '&', // Ampersand
    ];
    
    for c in non_word_chars {
        assert_eq!(is_word_character(c), false);
    }
}

#[test]
fn test_is_word_character_with_non_ascii() {
    // Test non-ASCII characters (should return false)
    let non_ascii_chars = vec![
        'é', // Latin small letter e with acute
        'ß', // Sharp S
        '中', // Chinese character
        '\u{1F600}', // Grinning face emoji
    ];
    
    for c in non_ascii_chars {
        assert_eq!(is_word_character(c), false);
    }
}


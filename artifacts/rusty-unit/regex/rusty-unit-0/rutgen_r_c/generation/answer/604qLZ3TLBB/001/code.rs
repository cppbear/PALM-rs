// Answer 0

#[test]
fn test_is_word_char_with_valid_word_characters() {
    struct CharWrapper(u32);
    impl CharWrapper {
        fn is_word_char(self) -> bool {
            char::from_u32(self.0).map_or(false, syntax::is_word_character)
        }
    }

    assert_eq!(CharWrapper(97).is_word_char(), true);  // 'a'
    assert_eq!(CharWrapper(122).is_word_char(), true); // 'z'
    assert_eq!(CharWrapper(65).is_word_char(), true);  // 'A'
    assert_eq!(CharWrapper(90).is_word_char(), true);  // 'Z'
    assert_eq!(CharWrapper(48).is_word_char(), true);  // '0'
    assert_eq!(CharWrapper(57).is_word_char(), true);  // '9'
}

#[test]
fn test_is_word_char_with_non_word_characters() {
    struct CharWrapper(u32);
    impl CharWrapper {
        fn is_word_char(self) -> bool {
            char::from_u32(self.0).map_or(false, syntax::is_word_character)
        }
    }

    assert_eq!(CharWrapper(32).is_word_char(), false);  // Space
    assert_eq!(CharWrapper(33).is_word_char(), false);  // '!'
    assert_eq!(CharWrapper(46).is_word_char(), false);  // '.'
    assert_eq!(CharWrapper(64).is_word_char(), false);  // '@'
}

#[test]
fn test_is_word_char_with_invalid_unicode() {
    struct CharWrapper(u32);
    impl CharWrapper {
        fn is_word_char(self) -> bool {
            char::from_u32(self.0).map_or(false, syntax::is_word_character)
        }
    }

    // Testing invalid Unicode code points
    assert_eq!(CharWrapper(0xD800).is_word_char(), false); // High surrogate
    assert_eq!(CharWrapper(0xDFFF).is_word_char(), false); // Low surrogate
}

#[test]
fn test_is_word_char_with_boundary_conditions() {
    struct CharWrapper(u32);
    impl CharWrapper {
        fn is_word_char(self) -> bool {
            char::from_u32(self.0).map_or(false, syntax::is_word_character)
        }
    }

    assert_eq!(CharWrapper(0).is_word_char(), false);        // Unicode U+0000
    assert_eq!(CharWrapper(0x10FFFF).is_word_char(), false); // Maximum valid Unicode code point
}


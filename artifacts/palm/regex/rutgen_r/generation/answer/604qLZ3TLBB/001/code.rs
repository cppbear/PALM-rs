// Answer 0

#[test]
fn test_is_word_char_valid_characters() {
    struct CharWrapper(u32);

    impl CharWrapper {
        pub fn is_word_char(self) -> bool {
            char::from_u32(self.0).map_or(false, |c| c.is_alphanumeric() || c == '_')
        }
    }

    assert!(CharWrapper(65).is_word_char()); // 'A'
    assert!(CharWrapper(97).is_word_char()); // 'a'
    assert!(CharWrapper(49).is_word_char()); // '1'
    assert!(CharWrapper(95).is_word_char()); // '_'
}

#[test]
fn test_is_word_char_invalid_characters() {
    struct CharWrapper(u32);

    impl CharWrapper {
        pub fn is_word_char(self) -> bool {
            char::from_u32(self.0).map_or(false, |c| c.is_alphanumeric() || c == '_')
        }
    }

    assert!(!CharWrapper(32).is_word_char()); // ' ' (space)
    assert!(!CharWrapper(36).is_word_char()); // '$'
    assert!(!CharWrapper(46).is_word_char()); // '.'
    assert!(!CharWrapper(255).is_word_char()); // Out of valid char range
}

#[test]
fn test_is_word_char_boundary_conditions() {
    struct CharWrapper(u32);

    impl CharWrapper {
        pub fn is_word_char(self) -> bool {
            char::from_u32(self.0).map_or(false, |c| c.is_alphanumeric() || c == '_')
        }
    }

    assert!(!CharWrapper(0).is_word_char()); // Invalid char
    assert!(!CharWrapper(1).is_word_char()); // Invalid char
    assert!(!CharWrapper(8).is_word_char()); // Backspace
    assert!(CharWrapper(122).is_word_char()); // 'z'
    assert!(CharWrapper(50000).is_word_char()); // Out of valid char range
}


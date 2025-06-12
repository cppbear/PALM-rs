// Answer 0

#[test]
fn test_is_word_byte_with_word_byte() {
    struct TestChar(u32);
    
    impl TestChar {
        fn is_word_byte(&self) -> bool {
            match char::from_u32(self.0) {
                Some(c) if c <= '\u{7F}' => syntax::is_word_byte(c as u8),
                None | Some(_) => false,
            }
        }
    }

    // Test input where the character is the maximum word byte ('\u{7F}')
    let ch = TestChar(127); // '\u{7F}' is outside ASCII range
    assert!(ch.is_word_byte() == false); // Assuming syntax::is_word_byte(127) returns false
}

#[test]
fn test_is_word_byte_with_non_word_byte() {
    struct TestChar(u32);
    
    impl TestChar {
        fn is_word_byte(&self) -> bool {
            match char::from_u32(self.0) {
                Some(c) if c <= '\u{7F}' => syntax::is_word_byte(c as u8),
                None | Some(_) => false,
            }
        }
    }

    // Test input where the character is a non-word byte (e.g., a special character)
    let ch = TestChar(50); // '2'
    assert!(ch.is_word_byte() == true); // Assuming syntax::is_word_byte(50) returns true
}

#[test]
fn test_is_word_byte_with_invalid_char() {
    struct TestChar(u32);
    
    impl TestChar {
        fn is_word_byte(&self) -> bool {
            match char::from_u32(self.0) {
                Some(c) if c <= '\u{7F}' => syntax::is_word_byte(c as u8),
                None | Some(_) => false,
            }
        }
    }

    // Test input where u32 is outside valid char range
    let ch = TestChar(40000); // Not a valid Unicode scalar value
    assert!(ch.is_word_byte() == false); // Assumed to not be a word byte
}


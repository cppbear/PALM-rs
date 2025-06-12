// Answer 0

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestChar(u32);

    impl TestChar {
        pub fn is_word_byte(self) -> bool {
            match char::from_u32(self.0) {
                Some(c) if c <= '\u{7F}' => syntax::is_word_byte(c as u8),
                None | Some(_) => false,
            }
        }
    }

    #[test]
    fn test_is_word_byte_none_case() {
        let input = TestChar(0xFFFFFFFF); // Out of valid char range, should return false
        assert_eq!(input.is_word_byte(), false);
    }

    #[test]
    fn test_is_word_byte_valid_case() {
        let input = TestChar(0x0041); // 'A', valid word byte
        // Assuming syntax::is_word_byte correctly recognizes 'A' which is a word byte.
        assert_eq!(input.is_word_byte(), true);
    }

    #[test]
    fn test_is_word_byte_non_ascii_case() {
        let input = TestChar(0x20AC); // '€', a non-ASCII character
        assert_eq!(input.is_word_byte(), false);
    }

    #[test]
    fn test_is_word_byte_boundary_case() {
        let input = TestChar(0x007F); // The boundary character, should still be a valid word byte
        assert_eq!(input.is_word_byte(), false); // ASCII DEL is not a word byte
    }
}


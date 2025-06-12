pub fn is_word_char(self) -> bool {
        char::from_u32(self.0).map_or(false, syntax::is_word_character)
    }
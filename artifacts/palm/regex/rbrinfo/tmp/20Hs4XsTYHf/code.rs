pub fn is_word_byte(self) -> bool {
        match char::from_u32(self.0) {
            Some(c) if c <= '\u{7F}' => syntax::is_word_byte(c as u8),
            None | Some(_) => false,
        }
    }
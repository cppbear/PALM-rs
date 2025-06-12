fn is_ascii_word(&self) -> bool {
        let b = match self.as_byte() {
            None => return false,
            Some(b) => b,
        };
        match b {
            b'A'...b'Z' | b'a'...b'z' | b'0'...b'9' | b'_' => true,
            _ => false,
        }
    }
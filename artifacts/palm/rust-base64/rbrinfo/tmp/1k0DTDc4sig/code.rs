const fn from_str_unchecked(alphabet: &str) -> Self {
        let mut symbols = [0_u8; ALPHABET_SIZE];
        let source_bytes = alphabet.as_bytes();

        // a way to copy that's allowed in const fn
        let mut index = 0;
        while index < ALPHABET_SIZE {
            symbols[index] = source_bytes[index];
            index += 1;
        }

        Self { symbols }
    }
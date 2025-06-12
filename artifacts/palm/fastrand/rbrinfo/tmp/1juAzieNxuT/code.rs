pub fn lowercase(&mut self) -> char {
        const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        *self.choice(CHARS).unwrap() as char
    }
pub fn uppercase(&mut self) -> char {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        *self.choice(CHARS).unwrap() as char
    }
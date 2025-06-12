pub fn matches(&self, byte: u8) -> bool {
        self.start <= byte && byte <= self.end
    }
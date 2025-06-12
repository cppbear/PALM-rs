pub fn as_bytes(&self) -> &'t [u8] {
        &self.text[self.start..self.end]
    }
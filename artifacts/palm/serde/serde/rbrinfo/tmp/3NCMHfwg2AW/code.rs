pub fn as_str(&self) -> &str {
        let slice = &self.bytes[..self.offset];
        unsafe { str::from_utf8_unchecked(slice) }
    }
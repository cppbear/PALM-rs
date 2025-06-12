fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.offset + s.len() > self.bytes.len() {
            Err(fmt::Error)
        } else {
            self.bytes[self.offset..self.offset + s.len()].copy_from_slice(s.as_bytes());
            self.offset += s.len();
            Ok(())
        }
    }
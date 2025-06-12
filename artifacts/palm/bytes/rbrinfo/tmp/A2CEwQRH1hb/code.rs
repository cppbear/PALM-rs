fn eq(&self, other: &str) -> bool {
        self.as_slice() == other.as_bytes()
    }
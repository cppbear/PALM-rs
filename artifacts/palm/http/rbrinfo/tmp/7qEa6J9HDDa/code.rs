fn eq(&self, other: &str) -> bool {
        self.inner == other.as_bytes()
    }
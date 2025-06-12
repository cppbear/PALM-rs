fn eq(&self, other: &Bytes) -> bool {
        self.as_slice() == other.as_slice()
    }
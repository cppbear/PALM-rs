fn eq(&self, other: &PathAndQuery) -> bool {
        self.as_str() == other.as_str()
    }
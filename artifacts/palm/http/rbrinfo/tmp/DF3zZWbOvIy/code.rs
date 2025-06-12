fn eq(&self, other: &PathAndQuery) -> bool {
        self == other.as_str()
    }
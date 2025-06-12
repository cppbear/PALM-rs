fn partial_cmp(&self, other: &PathAndQuery) -> Option<cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
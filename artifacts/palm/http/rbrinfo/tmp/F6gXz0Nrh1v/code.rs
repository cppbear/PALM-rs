fn partial_cmp(&self, other: &PathAndQuery) -> Option<cmp::Ordering> {
        self.partial_cmp(&other.as_str())
    }
fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_bytes())
    }
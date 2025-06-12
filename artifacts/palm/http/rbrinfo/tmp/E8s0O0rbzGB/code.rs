fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
        (*self.inner).partial_cmp(other.as_bytes())
    }
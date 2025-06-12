fn partial_cmp(&self, other: &String) -> Option<cmp::Ordering> {
        self.inner.partial_cmp(other.as_bytes())
    }
fn partial_cmp(&self, other: &Bytes) -> Option<cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
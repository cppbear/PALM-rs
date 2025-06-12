fn partial_cmp(&self, other: &String) -> Option<cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_bytes())
    }
fn partial_cmp(&self, other: &HeaderValue) -> Option<cmp::Ordering> {
        self.as_bytes().partial_cmp(other.as_bytes())
    }
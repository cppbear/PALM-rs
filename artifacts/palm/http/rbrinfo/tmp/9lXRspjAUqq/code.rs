fn partial_cmp(&self, other: &HeaderValue) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
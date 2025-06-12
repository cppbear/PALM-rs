fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
        self.as_str().partial_cmp(other)
    }
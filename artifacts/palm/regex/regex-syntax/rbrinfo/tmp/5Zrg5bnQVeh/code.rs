fn partial_cmp(&self, other: &Span) -> Option<Ordering> {
        Some(self.cmp(other))
    }
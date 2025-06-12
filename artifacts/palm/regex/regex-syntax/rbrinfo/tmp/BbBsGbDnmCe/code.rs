fn cmp(&self, other: &Span) -> Ordering {
        (&self.start, &self.end).cmp(&(&other.start, &other.end))
    }
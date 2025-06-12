fn cmp(&self, other: &Position) -> Ordering {
        self.offset.cmp(&other.offset)
    }
fn partial_cmp(&self, other: &char) -> Option<Ordering> {
        self.0.partial_cmp(&(*other as u32))
    }
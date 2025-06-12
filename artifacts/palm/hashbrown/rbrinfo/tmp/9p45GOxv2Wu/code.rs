fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.iter.size_hint();
        (lower.saturating_sub(self.other.len()), upper)
    }
pub(crate) fn capacity(&self) -> usize {
        Ord::min(self.indices.capacity(), self.entries.capacity())
    }
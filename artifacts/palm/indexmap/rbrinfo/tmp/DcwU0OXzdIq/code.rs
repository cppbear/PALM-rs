fn next_back(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.iter.next_back() {
            if self.other.contains(item) {
                return Some(item);
            }
        }
        None
    }
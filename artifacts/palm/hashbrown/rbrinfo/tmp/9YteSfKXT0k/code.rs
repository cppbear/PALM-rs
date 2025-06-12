fn size_hint(&self) -> (usize, Option<usize>) {
        (self.items, Some(self.items))
    }
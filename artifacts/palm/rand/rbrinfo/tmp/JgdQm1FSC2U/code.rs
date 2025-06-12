fn size_hint(&self) -> (usize, Option<usize>) {
        (self.indices.len(), Some(self.indices.len()))
    }
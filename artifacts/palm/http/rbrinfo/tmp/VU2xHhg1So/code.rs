fn size_hint(&self) -> (usize, Option<usize>) {
        match (self.front, self.back) {
            // Exactly 1 value...
            (Some(Cursor::Head), Some(Cursor::Head)) => (1, Some(1)),
            // At least 1...
            (Some(_), _) => (1, None),
            // No more values...
            (None, _) => (0, Some(0)),
        }
    }
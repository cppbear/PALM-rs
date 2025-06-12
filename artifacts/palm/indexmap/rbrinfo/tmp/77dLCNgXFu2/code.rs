fn size_hint(&self) -> (usize, Option<usize>) {
        (0, self.iter.size_hint().1)
    }
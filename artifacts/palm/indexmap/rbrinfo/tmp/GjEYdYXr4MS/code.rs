fn next(&mut self) -> Option<Self::Item> {
        Some(self.iter.next()?.0)
    }
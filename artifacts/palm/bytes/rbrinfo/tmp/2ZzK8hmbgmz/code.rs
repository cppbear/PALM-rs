fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = self.inner.remaining();
        (rem, Some(rem))
    }
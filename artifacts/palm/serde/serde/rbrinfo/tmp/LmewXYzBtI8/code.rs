fn size_hint(&self) -> Option<usize> {
        size_hint::from_bounds(&self.iter)
    }
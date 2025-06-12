fn size_hint(&self) -> Option<usize> {
        if self.0.is_some() {
            Some(2)
        } else if self.1.is_some() {
            Some(1)
        } else {
            Some(0)
        }
    }
fn next(&mut self) -> Option<T> {
        if self.first.is_some() {
            self.first.take()
        } else if let Some(ref mut extras) = self.next {
            extras.next()
        } else {
            None
        }
    }
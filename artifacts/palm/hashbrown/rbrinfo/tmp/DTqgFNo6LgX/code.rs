fn next(&mut self) -> Option<Self::Item> {
        self.inner.next(|val| (self.f)(val))
    }
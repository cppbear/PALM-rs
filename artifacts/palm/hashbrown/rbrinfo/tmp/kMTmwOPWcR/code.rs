fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next(|&mut (ref k, ())| (self.f)(k))
            .map(|(k, ())| k)
    }
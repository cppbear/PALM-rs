fn next(&mut self) -> Option<Self::Item> {
        self.inner.next(|&mut (ref k, ref mut v)| (self.f)(k, v))
    }
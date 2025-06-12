fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.inner.fold(init, |acc, x| unsafe {
            let (k, v) = x.as_mut();
            f(acc, (k, v))
        })
    }
fn fold<B, F>(self, init: B, f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        unsafe { self.iter.fold_impl(self.items, init, f) }
    }
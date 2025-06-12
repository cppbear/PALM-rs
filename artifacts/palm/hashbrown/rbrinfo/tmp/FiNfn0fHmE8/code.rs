fn fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item) -> B,
    {
        self.iter.fold(init, |acc, elt| {
            if self.other.contains(elt) {
                f(acc, elt)
            } else {
                acc
            }
        })
    }
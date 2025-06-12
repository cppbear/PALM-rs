fn into_iter(self) -> IntoIter<T, A> {
        IntoIter {
            inner: self.raw.into_iter(),
        }
    }
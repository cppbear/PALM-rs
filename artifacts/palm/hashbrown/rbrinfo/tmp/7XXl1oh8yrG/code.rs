fn into_iter(self) -> RawIntoIter<T, A> {
        unsafe {
            let iter = self.iter();
            self.into_iter_from(iter)
        }
    }
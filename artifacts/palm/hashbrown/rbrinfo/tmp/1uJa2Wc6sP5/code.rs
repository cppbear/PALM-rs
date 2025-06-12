fn into_iter(self) -> IntoIter<T, A> {
        IntoIter {
            iter: self.map.into_iter(),
        }
    }
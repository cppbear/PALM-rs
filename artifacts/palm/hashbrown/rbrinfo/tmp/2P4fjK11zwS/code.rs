fn into_iter(self) -> IntoIter<K, V, A> {
        IntoIter {
            inner: self.table.into_iter(),
        }
    }
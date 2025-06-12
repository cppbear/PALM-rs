fn into_iter(self) -> ValueIter<'a, T> {
        self.map.value_iter(self.index)
    }
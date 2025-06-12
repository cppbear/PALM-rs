pub fn iter(&self) -> ValueIter<'_, T> {
        self.map.value_iter(Some(self.index))
    }
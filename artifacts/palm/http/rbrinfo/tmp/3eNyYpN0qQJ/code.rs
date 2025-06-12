fn into_iter(self) -> ValueIterMut<'a, T> {
        self.map.value_iter_mut(self.index)
    }
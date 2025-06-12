pub fn iter_mut(&mut self) -> ValueIterMut<'_, T> {
        self.map.value_iter_mut(self.index)
    }
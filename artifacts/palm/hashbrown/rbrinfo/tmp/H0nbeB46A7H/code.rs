pub fn insert(&mut self, value: V) -> V {
        mem::replace(self.get_mut(), value)
    }
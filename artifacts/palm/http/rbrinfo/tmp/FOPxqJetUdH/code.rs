pub fn len(&self) -> usize {
        self.map.as_ref().map_or(0, |map| map.len())
    }
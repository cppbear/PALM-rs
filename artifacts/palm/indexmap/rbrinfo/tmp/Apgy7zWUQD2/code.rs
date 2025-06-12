pub fn split_off(&mut self, at: usize) -> Self
    where
        S: Clone,
    {
        Self {
            map: self.map.split_off(at),
        }
    }
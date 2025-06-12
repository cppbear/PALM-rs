pub fn split_off(&mut self, at: usize) -> Self
    where
        S: Clone,
    {
        Self {
            core: self.core.split_off(at),
            hash_builder: self.hash_builder.clone(),
        }
    }
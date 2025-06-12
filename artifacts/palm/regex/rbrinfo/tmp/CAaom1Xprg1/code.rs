fn deref(&self) -> &Self::Target {
        &self.dense[0..self.size]
    }
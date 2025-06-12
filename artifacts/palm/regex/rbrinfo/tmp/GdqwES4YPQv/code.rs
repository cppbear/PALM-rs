fn approximate_size(&self) -> usize {
        (self.dense.len() * mem::size_of::<u8>())
        + (self.sparse.len() * mem::size_of::<bool>())
    }
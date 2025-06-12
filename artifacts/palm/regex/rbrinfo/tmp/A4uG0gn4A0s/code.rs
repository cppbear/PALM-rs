fn approximate_size(&self) -> usize {
        (self.pattern.len() * mem::size_of::<u8>())
            + (256 * mem::size_of::<usize>()) // skip table
    }
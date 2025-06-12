fn approximate_size(&self) -> usize {
        self.pat.len() * mem::size_of::<u8>()
    }
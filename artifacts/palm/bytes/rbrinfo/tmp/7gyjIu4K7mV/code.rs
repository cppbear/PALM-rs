fn copy_to_bytes(&mut self, len: usize) -> Self {
        self.split_to(len)
    }
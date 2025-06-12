pub fn set_limit_size(&mut self, size: usize) -> &mut Literals {
        self.limit_size = size;
        self
    }
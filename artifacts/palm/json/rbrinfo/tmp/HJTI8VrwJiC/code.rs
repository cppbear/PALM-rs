pub fn byte_offset(&self) -> usize {
        self.start_of_line + self.col
    }
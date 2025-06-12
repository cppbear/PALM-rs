fn kind(&self) -> usize {
        self.data as usize & KIND_MASK
    }
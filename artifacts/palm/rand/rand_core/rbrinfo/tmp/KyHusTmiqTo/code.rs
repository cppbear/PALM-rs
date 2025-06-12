fn next_u32(&mut self) -> u32 {
        self.0.try_next_u32().unwrap()
    }
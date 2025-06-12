fn next_u64(&mut self) -> u64 {
        self.0.try_next_u64().unwrap()
    }
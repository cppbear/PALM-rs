fn next_u32(&mut self) -> u32 {
        // The lowest bits have some linear dependencies, so we use the
        // upper bits instead.
        let val = self.next_u64();
        (val >> 32) as u32
    }
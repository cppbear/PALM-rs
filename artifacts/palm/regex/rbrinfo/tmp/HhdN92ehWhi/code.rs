fn caps(&mut self, pc: usize) -> &mut [Option<usize>] {
        let i = pc * self.slots_per_thread;
        &mut self.caps[i..i + self.slots_per_thread]
    }
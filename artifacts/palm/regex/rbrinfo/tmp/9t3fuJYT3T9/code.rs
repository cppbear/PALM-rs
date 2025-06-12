fn resize(&mut self, num_insts: usize, ncaps: usize) {
        if num_insts == self.set.capacity() {
            return;
        }
        self.slots_per_thread = ncaps * 2;
        self.set = SparseSet::new(num_insts);
        self.caps = vec![None; self.slots_per_thread * num_insts];
    }
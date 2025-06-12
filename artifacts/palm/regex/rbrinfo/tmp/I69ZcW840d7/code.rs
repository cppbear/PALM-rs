fn next(&self, si: StatePtr, cls: usize) -> StatePtr {
        self.table[si as usize + cls]
    }
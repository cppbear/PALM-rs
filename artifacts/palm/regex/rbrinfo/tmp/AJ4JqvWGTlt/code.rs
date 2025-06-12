fn set_next(&mut self, si: StatePtr, cls: usize, next: StatePtr) {
        self.table[si as usize + cls] = next;
    }
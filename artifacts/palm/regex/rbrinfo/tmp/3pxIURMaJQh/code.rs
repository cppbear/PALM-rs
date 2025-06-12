unsafe fn next_unchecked(&self, si: StatePtr, cls: usize) -> StatePtr {
        debug_assert!((si as usize) < self.table.len());
        debug_assert!(cls < self.num_byte_classes);
        *self.table.get_unchecked(si as usize + cls)
    }
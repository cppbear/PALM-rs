pub fn remove_entry(self) -> (K, V) {
        unsafe { self.table.table.remove(self.elem).0 }
    }
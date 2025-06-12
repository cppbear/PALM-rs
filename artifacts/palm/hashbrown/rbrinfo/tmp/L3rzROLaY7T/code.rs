pub fn remove_entry(self) -> (K, V) {
        unsafe { self.table.remove(self.elem).0 }
    }
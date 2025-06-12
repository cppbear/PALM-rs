pub fn truncate(&mut self, len: usize) {
        if len < self.len {
            // The Vec "promotable" vtables do not store the capacity,
            // so we cannot truncate while using this repr. We *have* to
            // promote using `split_off` so the capacity can be stored.
            if self.vtable as *const Vtable == &PROMOTABLE_EVEN_VTABLE
                || self.vtable as *const Vtable == &PROMOTABLE_ODD_VTABLE
            {
                drop(self.split_off(len));
            } else {
                self.len = len;
            }
        }
    }
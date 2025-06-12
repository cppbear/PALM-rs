pub fn clear(&mut self) {
        if self.is_empty() {
            // Special case empty table to avoid surprising O(capacity) time.
            return;
        }
        // Ensure that the table is reset even if one of the drops panic
        let mut self_ = guard(self, |self_| self_.clear_no_drop());
        unsafe {
            // SAFETY: ScopeGuard sets to zero the `items` field of the table
            // even in case of panic during the dropping of the elements so
            // that there will be no double drop of the elements.
            self_.table.drop_elements::<T>();
        }
    }
unsafe fn prepare_insert_slot(&mut self, hash: u64) -> (usize, Tag) {
        // SAFETY: Caller of this function ensures that the control bytes are properly initialized.
        let index: usize = self.find_insert_slot(hash).index;
        // SAFETY:
        // 1. The `find_insert_slot` function either returns an `index` less than or
        //    equal to `self.buckets() = self.bucket_mask + 1` of the table, or never
        //    returns if it cannot find an empty or deleted slot.
        // 2. The caller of this function guarantees that the table has already been
        //    allocated
        let old_ctrl = *self.ctrl(index);
        self.set_ctrl_hash(index, hash);
        (index, old_ctrl)
    }
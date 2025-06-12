pub unsafe fn insert_in_slot(&mut self, hash: u64, slot: InsertSlot, value: T) -> Bucket<T> {
        let old_ctrl = *self.table.ctrl(slot.index);
        self.table.record_item_insert_at(slot.index, old_ctrl, hash);

        let bucket = self.bucket(slot.index);
        bucket.write(value);
        bucket
    }
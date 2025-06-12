pub fn insert(self, value: T) -> OccupiedEntry<'a, T, A> {
        let bucket = unsafe {
            self.table
                .raw
                .insert_in_slot(self.hash, self.insert_slot, value)
        };
        OccupiedEntry {
            hash: self.hash,
            bucket,
            table: self.table,
        }
    }
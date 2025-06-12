pub fn remove(self) -> (T, VacantEntry<'a, T, A>) {
        let (val, slot) = unsafe { self.table.raw.remove(self.bucket) };
        (
            val,
            VacantEntry {
                hash: self.hash,
                insert_slot: slot,
                table: self.table,
            },
        )
    }
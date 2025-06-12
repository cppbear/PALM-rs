pub fn entry(
        &mut self,
        hash: u64,
        eq: impl FnMut(&T) -> bool,
        hasher: impl Fn(&T) -> u64,
    ) -> Entry<'_, T, A> {
        match self.raw.find_or_find_insert_slot(hash, eq, hasher) {
            Ok(bucket) => Entry::Occupied(OccupiedEntry {
                hash,
                bucket,
                table: self,
            }),
            Err(insert_slot) => Entry::Vacant(VacantEntry {
                hash,
                insert_slot,
                table: self,
            }),
        }
    }
pub fn insert_unique(
        &mut self,
        hash: u64,
        value: T,
        hasher: impl Fn(&T) -> u64,
    ) -> OccupiedEntry<'_, T, A> {
        let bucket = self.raw.insert(hash, value, hasher);
        OccupiedEntry {
            hash,
            bucket,
            table: self,
        }
    }
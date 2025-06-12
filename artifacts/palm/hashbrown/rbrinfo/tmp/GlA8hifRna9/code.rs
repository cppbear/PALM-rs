pub fn find_entry(
        &mut self,
        hash: u64,
        eq: impl FnMut(&T) -> bool,
    ) -> Result<OccupiedEntry<'_, T, A>, AbsentEntry<'_, T, A>> {
        match self.raw.find(hash, eq) {
            Some(bucket) => Ok(OccupiedEntry {
                hash,
                bucket,
                table: self,
            }),
            None => Err(AbsentEntry { table: self }),
        }
    }
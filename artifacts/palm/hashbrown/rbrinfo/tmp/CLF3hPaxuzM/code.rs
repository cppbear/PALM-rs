fn search<F>(self, hash: u64, mut is_match: F) -> RawEntryMut<'a, K, V, S, A>
    where
        for<'b> F: FnMut(&'b K) -> bool,
    {
        match self.map.table.find(hash, |(k, _)| is_match(k)) {
            Some(elem) => RawEntryMut::Occupied(RawOccupiedEntryMut {
                elem,
                table: &mut self.map.table,
                hash_builder: &self.map.hash_builder,
            }),
            None => RawEntryMut::Vacant(RawVacantEntryMut {
                table: &mut self.map.table,
                hash_builder: &self.map.hash_builder,
            }),
        }
    }
pub fn entry_ref<'a, 'b, Q>(&'a mut self, key: &'b Q) -> EntryRef<'a, 'b, K, Q, V, S, A>
    where
        Q: Hash + Equivalent<K> + ?Sized,
    {
        let hash = make_hash::<Q, S>(&self.hash_builder, key);
        if let Some(elem) = self.table.find(hash, equivalent_key(key)) {
            EntryRef::Occupied(OccupiedEntry {
                hash,
                elem,
                table: self,
            })
        } else {
            EntryRef::Vacant(VacantEntryRef {
                hash,
                key,
                table: self,
            })
        }
    }
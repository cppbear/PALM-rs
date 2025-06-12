pub fn entry(&mut self, key: K) -> Entry<'_, K, V, S, A> {
        let hash = make_hash::<K, S>(&self.hash_builder, &key);
        if let Some(elem) = self.table.find(hash, equivalent_key(&key)) {
            Entry::Occupied(OccupiedEntry {
                hash,
                elem,
                table: self,
            })
        } else {
            Entry::Vacant(VacantEntry {
                hash,
                key,
                table: self,
            })
        }
    }
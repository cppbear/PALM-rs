pub(crate) fn replace_full(
        &mut self,
        hash: HashValue,
        key: K,
        value: V,
    ) -> (usize, Option<(K, V)>)
    where
        K: Eq,
    {
        let eq = equivalent(&key, &self.entries);
        let hasher = get_hash(&self.entries);
        match self.indices.entry(hash.get(), eq, hasher) {
            hash_table::Entry::Occupied(entry) => {
                let i = *entry.get();
                let entry = &mut self.entries[i];
                let kv = (
                    mem::replace(&mut entry.key, key),
                    mem::replace(&mut entry.value, value),
                );
                (i, Some(kv))
            }
            hash_table::Entry::Vacant(entry) => {
                let i = self.entries.len();
                entry.insert(i);
                self.push_entry(hash, key, value);
                debug_assert_eq!(self.indices.len(), self.entries.len());
                (i, None)
            }
        }
    }
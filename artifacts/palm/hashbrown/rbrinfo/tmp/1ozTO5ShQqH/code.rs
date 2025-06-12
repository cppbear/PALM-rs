pub fn replace_entry_with<F>(self, f: F) -> RawEntryMut<'a, K, V, S, A>
    where
        F: FnOnce(&K, V) -> Option<V>,
    {
        unsafe {
            let still_occupied = self
                .table
                .replace_bucket_with(self.elem.clone(), |(key, value)| {
                    f(&key, value).map(|new_value| (key, new_value))
                });

            if still_occupied {
                RawEntryMut::Occupied(self)
            } else {
                RawEntryMut::Vacant(RawVacantEntryMut {
                    table: self.table,
                    hash_builder: self.hash_builder,
                })
            }
        }
    }
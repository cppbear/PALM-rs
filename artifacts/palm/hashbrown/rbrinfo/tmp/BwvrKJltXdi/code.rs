pub fn replace_entry_with<F>(self, f: F) -> Entry<'a, K, V, S, A>
    where
        F: FnOnce(&K, V) -> Option<V>,
    {
        unsafe {
            let mut spare_key = None;

            self.table
                .table
                .replace_bucket_with(self.elem.clone(), |(key, value)| {
                    if let Some(new_value) = f(&key, value) {
                        Some((key, new_value))
                    } else {
                        spare_key = Some(key);
                        None
                    }
                });

            if let Some(key) = spare_key {
                Entry::Vacant(VacantEntry {
                    hash: self.hash,
                    key,
                    table: self.table,
                })
            } else {
                Entry::Occupied(self)
            }
        }
    }
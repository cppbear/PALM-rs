pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        let hash = make_hash::<K, S>(&self.hash_builder, &k);
        match self.find_or_find_insert_slot(hash, &k) {
            Ok(bucket) => Some(mem::replace(unsafe { &mut bucket.as_mut().1 }, v)),
            Err(slot) => {
                unsafe {
                    self.table.insert_in_slot(hash, slot, (k, v));
                }
                None
            }
        }
    }
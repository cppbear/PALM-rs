pub fn get_or_insert(&mut self, value: T) -> &T {
        let hash = make_hash(&self.map.hash_builder, &value);
        let bucket = match self.map.find_or_find_insert_slot(hash, &value) {
            Ok(bucket) => bucket,
            Err(slot) => unsafe { self.map.table.insert_in_slot(hash, slot, (value, ())) },
        };
        unsafe { &bucket.as_ref().0 }
    }
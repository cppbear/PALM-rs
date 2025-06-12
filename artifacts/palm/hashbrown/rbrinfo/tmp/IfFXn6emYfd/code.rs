pub fn get_or_insert_with<Q, F>(&mut self, value: &Q, f: F) -> &T
    where
        Q: Hash + Equivalent<T> + ?Sized,
        F: FnOnce(&Q) -> T,
    {
        let hash = make_hash(&self.map.hash_builder, value);
        let bucket = match self.map.find_or_find_insert_slot(hash, value) {
            Ok(bucket) => bucket,
            Err(slot) => {
                let new = f(value);
                assert!(value.equivalent(&new), "new value is not equivalent");
                unsafe { self.map.table.insert_in_slot(hash, slot, (new, ())) }
            }
        };
        unsafe { &bucket.as_ref().0 }
    }
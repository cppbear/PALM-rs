pub fn replace(&mut self, value: T) -> Option<T> {
        let hash = make_hash(&self.map.hash_builder, &value);
        match self.map.find_or_find_insert_slot(hash, &value) {
            Ok(bucket) => Some(mem::replace(unsafe { &mut bucket.as_mut().0 }, value)),
            Err(slot) => {
                unsafe {
                    self.map.table.insert_in_slot(hash, slot, (value, ()));
                }
                None
            }
        }
    }
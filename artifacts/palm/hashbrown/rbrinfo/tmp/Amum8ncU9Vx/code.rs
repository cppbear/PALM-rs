fn bitxor_assign(&mut self, rhs: &HashSet<T, S, A>) {
        for item in rhs {
            let hash = make_hash(&self.map.hash_builder, item);
            match self.map.find_or_find_insert_slot(hash, item) {
                Ok(bucket) => unsafe {
                    self.map.table.remove(bucket);
                },
                Err(slot) => unsafe {
                    self.map
                        .table
                        .insert_in_slot(hash, slot, (item.clone(), ()));
                },
            }
        }
    }
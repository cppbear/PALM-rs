fn push_entry(&mut self, hash: HashValue, key: K, value: V) {
        if self.entries.len() == self.entries.capacity() {
            // Reserve our own capacity synced to the indices,
            // rather than letting `Vec::push` just double it.
            self.borrow_mut().reserve_entries(1);
        }
        self.entries.push(Bucket { hash, key, value });
    }
fn rebuild_hash_table(&mut self) {
        self.indices.clear();
        insert_bulk_no_grow(&mut self.indices, &self.entries);
    }
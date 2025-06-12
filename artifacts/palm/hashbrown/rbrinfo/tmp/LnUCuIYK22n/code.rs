fn clone(&self) -> Self {
        HashMap {
            hash_builder: self.hash_builder.clone(),
            table: self.table.clone(),
        }
    }
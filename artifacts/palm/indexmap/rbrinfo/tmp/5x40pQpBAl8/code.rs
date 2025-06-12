fn clone(&self) -> Self {
        IndexMap {
            core: self.core.clone(),
            hash_builder: self.hash_builder.clone(),
        }
    }
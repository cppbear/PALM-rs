fn clone(&self) -> Self {
        IndexSet {
            map: self.map.clone(),
        }
    }
fn clone(&self) -> Self {
        HashSet {
            map: self.map.clone(),
        }
    }
fn clone(&self) -> Self {
        Self {
            iter: self.iter.clone(),
            items: self.items,
        }
    }
fn clone(&self) -> Self {
        Values {
            iter: self.iter.clone(),
        }
    }
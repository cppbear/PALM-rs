fn clone(&self) -> Self {
        Keys {
            iter: self.iter.clone(),
        }
    }
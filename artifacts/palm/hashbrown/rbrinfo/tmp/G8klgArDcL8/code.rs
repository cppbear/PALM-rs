fn clone(&self) -> Self {
        Union {
            iter: self.iter.clone(),
        }
    }
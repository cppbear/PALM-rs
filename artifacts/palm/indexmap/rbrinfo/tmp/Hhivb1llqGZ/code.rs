fn clone(&self) -> Self {
        Iter {
            iter: self.iter.clone(),
        }
    }
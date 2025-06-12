fn clone(&self) -> Self {
        SymmetricDifference {
            iter: self.iter.clone(),
        }
    }
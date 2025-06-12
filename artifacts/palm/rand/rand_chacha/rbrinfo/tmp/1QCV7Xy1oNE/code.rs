fn clone(&self) -> Self {
        let mut new = Self::default();
        new.0.copy_from_slice(&self.0);
        new
    }
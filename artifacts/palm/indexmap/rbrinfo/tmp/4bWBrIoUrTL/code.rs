fn clone(&self) -> Self {
        let mut new = Self::new();
        new.clone_from(self);
        new
    }
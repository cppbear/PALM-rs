fn clone(&self) -> Self {
            match self.get() {
                Some(value) => OnceBox::with_value(Box::new(value.clone())),
                None => OnceBox::new(),
            }
        }
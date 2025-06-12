fn clone(&self) -> OnceCell<T> {
            match self.get() {
                Some(value) => Self::with_value(value.clone()),
                None => Self::new(),
            }
        }
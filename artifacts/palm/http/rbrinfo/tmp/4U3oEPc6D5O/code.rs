fn map<F>(self, func: F) -> Self
    where
        F: FnOnce(Parts) -> Result<Parts, crate::Error>,
    {
        Builder {
            parts: self.parts.and_then(func),
        }
    }
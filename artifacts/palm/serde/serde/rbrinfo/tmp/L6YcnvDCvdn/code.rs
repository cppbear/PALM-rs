fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.lock() {
            Ok(locked) => locked.serialize(serializer),
            Err(_) => Err(S::Error::custom("lock poison error while serializing")),
        }
    }
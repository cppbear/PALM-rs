fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.try_borrow() {
            Ok(value) => value.serialize(serializer),
            Err(_) => Err(S::Error::custom("already mutably borrowed")),
        }
    }
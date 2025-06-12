fn next_value_seed<T>(&mut self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.pending_content.take() {
            Some(value) => seed.deserialize(ContentDeserializer::new(value)),
            None => Err(Error::custom("value is missing")),
        }
    }
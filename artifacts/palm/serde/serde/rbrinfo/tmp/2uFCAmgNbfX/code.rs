fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if let Some(k) = self.0.take() {
            seed.deserialize(k.into_deserializer()).map(Some)
        } else if let Some(v) = self.1.take() {
            seed.deserialize(v.into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }
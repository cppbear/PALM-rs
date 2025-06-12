fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        for entry in self.iter.by_ref() {
            if let Some((key, content)) = flat_map_take_entry(entry, self.fields) {
                self.pending_content = Some(content);
                return seed.deserialize(ContentDeserializer::new(key)).map(Some);
            }
        }
        Ok(None)
    }
fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        for item in &mut self.iter {
            // Items in the vector are nulled out when used by a struct.
            if let Some((ref key, ref content)) = *item {
                // Do not take(), instead borrow this entry. The internally tagged
                // enum does its own buffering so we can't tell whether this entry
                // is going to be consumed. Borrowing here leaves the entry
                // available for later flattened fields.
                self.pending_content = Some(content);
                return seed.deserialize(ContentRefDeserializer::new(key)).map(Some);
            }
        }
        Ok(None)
    }
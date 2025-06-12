fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.next_pair() {
            Some((k, v)) => {
                let de = PairDeserializer(k, v, PhantomData);
                seed.deserialize(de).map(Some)
            }
            None => Ok(None),
        }
    }
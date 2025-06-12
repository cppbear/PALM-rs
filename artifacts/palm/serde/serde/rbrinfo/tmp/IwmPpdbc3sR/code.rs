fn newtype_variant_seed<T>(mut self, seed: T) -> Result<T::Value, Self::Error>
        where
            T: DeserializeSeed<'de>,
        {
            self.map.next_value_seed(seed)
        }
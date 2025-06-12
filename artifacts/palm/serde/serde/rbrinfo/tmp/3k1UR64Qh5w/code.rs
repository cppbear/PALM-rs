fn variant_seed<T>(mut self, seed: T) -> Result<(T::Value, Self::Variant), Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match tri!(self.map.next_key_seed(seed)) {
            Some(key) => Ok((key, private::map_as_enum(self.map))),
            None => Err(de::Error::invalid_type(de::Unexpected::Map, &"enum")),
        }
    }
fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, E>
        where
            T: de::DeserializeSeed<'de>,
        {
            match self.value {
                Some(value) => seed.deserialize(ContentDeserializer::new(value)),
                None => Err(de::Error::invalid_type(
                    de::Unexpected::UnitVariant,
                    &"newtype variant",
                )),
            }
        }
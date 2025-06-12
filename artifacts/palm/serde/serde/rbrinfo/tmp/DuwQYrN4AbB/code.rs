fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
        where
            T: de::DeserializeSeed<'de>,
        {
            Err(de::Error::invalid_type(
                Unexpected::UnitVariant,
                &"newtype variant",
            ))
        }
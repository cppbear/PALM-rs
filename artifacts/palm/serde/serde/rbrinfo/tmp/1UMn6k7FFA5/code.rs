fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, E>
        where
            T: de::DeserializeSeed<'de>,
        {
            match self.value {
                // Covered by tests/test_annotations.rs
                //      test_partially_untagged_enum_desugared
                //      test_partially_untagged_enum_generic
                // Covered by tests/test_enum_untagged.rs
                //      newtype_enum::newtype
                Some(value) => seed.deserialize(ContentRefDeserializer::new(value)),
                None => Err(de::Error::invalid_type(
                    de::Unexpected::UnitVariant,
                    &"newtype variant",
                )),
            }
        }
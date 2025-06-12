fn unit_variant(self) -> Result<(), E> {
            match self.value {
                Some(value) => de::Deserialize::deserialize(ContentRefDeserializer::new(value)),
                // Covered by tests/test_annotations.rs
                //      test_partially_untagged_adjacently_tagged_enum
                // Covered by tests/test_enum_untagged.rs
                //      newtype_enum::unit
                None => Ok(()),
            }
        }
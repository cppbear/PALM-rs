fn struct_variant<V>(
            self,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            match self.value {
                // Covered by tests/test_enum_untagged.rs
                //      newtype_enum::struct_from_map
                Some(Content::Map(v)) => visit_content_map_ref(v, visitor),
                // Covered by tests/test_enum_untagged.rs
                //      newtype_enum::struct_from_seq
                //      newtype_enum::empty_struct_from_seq
                Some(Content::Seq(v)) => visit_content_seq_ref(v, visitor),
                Some(other) => Err(de::Error::invalid_type(
                    other.unexpected(),
                    &"struct variant",
                )),
                None => Err(de::Error::invalid_type(
                    de::Unexpected::UnitVariant,
                    &"struct variant",
                )),
            }
        }
fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            match self.value {
                // Covered by tests/test_annotations.rs
                //      test_partially_untagged_enum
                //      test_partially_untagged_enum_desugared
                // Covered by tests/test_enum_untagged.rs
                //      newtype_enum::tuple0
                //      newtype_enum::tuple2
                Some(Content::Seq(v)) => visit_content_seq_ref(v, visitor),
                Some(other) => Err(de::Error::invalid_type(
                    other.unexpected(),
                    &"tuple variant",
                )),
                None => Err(de::Error::invalid_type(
                    de::Unexpected::UnitVariant,
                    &"tuple variant",
                )),
            }
        }
fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            match self.value {
                Some(Content::Seq(v)) => {
                    de::Deserializer::deserialize_any(SeqDeserializer::new(v.into_iter()), visitor)
                }
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
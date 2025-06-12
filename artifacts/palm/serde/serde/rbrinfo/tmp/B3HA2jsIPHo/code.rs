fn struct_variant<V>(
            self,
            _fields: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            match self.value {
                Some(Content::Map(v)) => {
                    de::Deserializer::deserialize_any(MapDeserializer::new(v.into_iter()), visitor)
                }
                Some(Content::Seq(v)) => {
                    de::Deserializer::deserialize_any(SeqDeserializer::new(v.into_iter()), visitor)
                }
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
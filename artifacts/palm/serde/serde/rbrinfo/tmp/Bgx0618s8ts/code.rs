fn struct_variant<V>(
            self,
            _fields: &'static [&'static str],
            _visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: de::Visitor<'de>,
        {
            Err(de::Error::invalid_type(
                Unexpected::UnitVariant,
                &"struct variant",
            ))
        }
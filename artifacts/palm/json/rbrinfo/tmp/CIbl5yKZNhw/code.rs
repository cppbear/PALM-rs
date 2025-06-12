fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let mut iter = self.into_iter();
        let (variant, value) = match iter.next() {
            Some(v) => v,
            None => {
                return Err(serde::de::Error::invalid_value(
                    Unexpected::Map,
                    &"map with a single key",
                ));
            }
        };
        // enums are encoded in json as maps with a single key:value pair
        if iter.next().is_some() {
            return Err(serde::de::Error::invalid_value(
                Unexpected::Map,
                &"map with a single key",
            ));
        }

        visitor.visit_enum(EnumRefDeserializer {
            variant,
            value: Some(value),
        })
    }
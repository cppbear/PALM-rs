fn deserialize_enum<V>(
            self,
            _name: &str,
            _variants: &'static [&'static str],
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            let (variant, value) = match *self.content {
                Content::Map(ref value) => {
                    let mut iter = value.iter();
                    let (variant, value) = match iter.next() {
                        Some(v) => v,
                        None => {
                            return Err(de::Error::invalid_value(
                                de::Unexpected::Map,
                                &"map with a single key",
                            ));
                        }
                    };
                    // enums are encoded in json as maps with a single key:value pair
                    if iter.next().is_some() {
                        return Err(de::Error::invalid_value(
                            de::Unexpected::Map,
                            &"map with a single key",
                        ));
                    }
                    (variant, Some(value))
                }
                ref s @ Content::String(_) | ref s @ Content::Str(_) => (s, None),
                ref other => {
                    return Err(de::Error::invalid_type(
                        other.unexpected(),
                        &"string or map",
                    ));
                }
            };

            visitor.visit_enum(EnumRefDeserializer {
                variant,
                value,
                err: PhantomData,
            })
        }
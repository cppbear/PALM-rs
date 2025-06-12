fn deserialize_newtype_struct<V>(self, name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        #[cfg(feature = "raw_value")]
        {
            if name == crate::raw::TOKEN {
                return self.de.deserialize_raw_value(visitor);
            }
        }

        let _ = name;
        visitor.visit_newtype_struct(self)
    }
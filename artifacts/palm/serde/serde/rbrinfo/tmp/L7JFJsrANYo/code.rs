fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.content {
                Content::Unit => visitor.visit_unit(),

                // Allow deserializing newtype variant containing unit.
                //
                //     #[derive(Deserialize)]
                //     #[serde(tag = "result")]
                //     enum Response<T> {
                //         Success(T),
                //     }
                //
                // We want {"result":"Success"} to deserialize into Response<()>.
                Content::Map(ref v) if v.is_empty() => visitor.visit_unit(),
                _ => Err(self.invalid_type(&visitor)),
            }
        }
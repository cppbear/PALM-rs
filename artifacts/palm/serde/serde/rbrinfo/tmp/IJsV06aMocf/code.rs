fn deserialize_unit_struct<V>(
            self,
            _name: &'static str,
            visitor: V,
        ) -> Result<V::Value, Self::Error>
        where
            V: Visitor<'de>,
        {
            match self.content {
                // As a special case, allow deserializing untagged newtype
                // variant containing unit struct.
                //
                //     #[derive(Deserialize)]
                //     struct Info;
                //
                //     #[derive(Deserialize)]
                //     #[serde(tag = "topic")]
                //     enum Message {
                //         Info(Info),
                //     }
                //
                // We want {"topic":"Info"} to deserialize even though
                // ordinarily unit structs do not deserialize from empty map/seq.
                Content::Map(ref v) if v.is_empty() => visitor.visit_unit(),
                Content::Seq(ref v) if v.is_empty() => visitor.visit_unit(),
                _ => self.deserialize_any(visitor),
            }
        }
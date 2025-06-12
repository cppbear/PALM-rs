fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
        where
            V: MapAccess<'de>,
        {
            let mut vec =
                Vec::<(Content, Content)>::with_capacity(
                    size_hint::cautious::<(Content, Content)>(visitor.size_hint()),
                );
            while let Some(kv) = tri!(visitor.next_entry()) {
                vec.push(kv);
            }
            Ok(Content::Map(vec))
        }
fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            let v = tri!(Deserialize::deserialize(deserializer));
            Ok(Content::Newtype(Box::new(v)))
        }
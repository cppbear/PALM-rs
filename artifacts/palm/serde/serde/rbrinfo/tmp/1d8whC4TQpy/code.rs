fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            let v = tri!(Deserialize::deserialize(deserializer));
            Ok(Content::Some(Box::new(v)))
        }
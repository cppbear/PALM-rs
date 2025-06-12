fn visit_unit<F>(self) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            Ok(Content::Unit)
        }
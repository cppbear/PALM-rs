fn visit_none<F>(self) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            Ok(Content::None)
        }
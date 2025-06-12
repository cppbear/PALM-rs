fn visit_bool<F>(self, value: bool) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            Ok(Content::Bool(value))
        }
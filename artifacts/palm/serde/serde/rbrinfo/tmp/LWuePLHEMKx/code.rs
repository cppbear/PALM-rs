fn visit_string<F>(self, value: String) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            Ok(Content::String(value))
        }
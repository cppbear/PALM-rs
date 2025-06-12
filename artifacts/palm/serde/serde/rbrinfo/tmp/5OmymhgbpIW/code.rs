fn visit_borrowed_str<F>(self, value: &'de str) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            Ok(Content::Str(value))
        }
fn visit_borrowed_bytes<F>(self, value: &'de [u8]) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            Ok(Content::Bytes(value))
        }
fn visit_u8<F>(self, value: u8) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            Ok(Content::U8(value))
        }
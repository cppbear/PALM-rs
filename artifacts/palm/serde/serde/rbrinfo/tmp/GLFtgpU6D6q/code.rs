fn visit_u32<F>(self, value: u32) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            Ok(Content::U32(value))
        }
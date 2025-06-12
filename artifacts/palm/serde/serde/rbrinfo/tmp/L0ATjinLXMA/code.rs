fn visit_i8<F>(self, value: i8) -> Result<Self::Value, F>
        where
            F: de::Error,
        {
            Ok(Content::I8(value))
        }
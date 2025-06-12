fn visit_str<E>(self, field: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_bytes(field.as_bytes())
        }
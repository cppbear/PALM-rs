fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let _ = s;
        Ok(IgnoredAny)
    }
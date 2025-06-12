fn visit_bytes<E>(self, bytes: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let _ = bytes;
        Ok(IgnoredAny)
    }
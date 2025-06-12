fn visit_u64<E>(self, x: u64) -> Result<Self::Value, E> {
        let _ = x;
        Ok(IgnoredAny)
    }
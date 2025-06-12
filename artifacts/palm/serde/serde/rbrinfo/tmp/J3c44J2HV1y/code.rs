fn visit_i64<E>(self, x: i64) -> Result<Self::Value, E> {
        let _ = x;
        Ok(IgnoredAny)
    }
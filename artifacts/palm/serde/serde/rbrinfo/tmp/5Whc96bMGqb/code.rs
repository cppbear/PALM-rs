fn visit_bool<E>(self, x: bool) -> Result<Self::Value, E> {
        let _ = x;
        Ok(IgnoredAny)
    }
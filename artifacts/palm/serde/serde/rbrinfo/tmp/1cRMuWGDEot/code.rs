fn visit_i128<E>(self, x: i128) -> Result<Self::Value, E> {
        let _ = x;
        Ok(IgnoredAny)
    }
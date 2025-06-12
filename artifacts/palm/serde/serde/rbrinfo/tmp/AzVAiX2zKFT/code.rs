fn visit_u128<E>(self, x: u128) -> Result<Self::Value, E> {
        let _ = x;
        Ok(IgnoredAny)
    }
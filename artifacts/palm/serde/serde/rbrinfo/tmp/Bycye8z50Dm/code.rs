fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(IgnoredAny)
    }
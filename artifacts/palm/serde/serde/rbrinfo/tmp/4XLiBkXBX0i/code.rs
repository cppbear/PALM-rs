fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(IgnoredAny)
    }
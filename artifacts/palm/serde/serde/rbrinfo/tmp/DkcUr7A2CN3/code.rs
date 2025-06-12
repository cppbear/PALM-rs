fn visit_f64<E>(self, x: f64) -> Result<Self::Value, E> {
        let _ = x;
        Ok(IgnoredAny)
    }
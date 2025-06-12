fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        *self.0 = v;
        Ok(())
    }
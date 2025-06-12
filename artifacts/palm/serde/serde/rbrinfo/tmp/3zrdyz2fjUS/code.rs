fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Unsigned(v), &self))
    }
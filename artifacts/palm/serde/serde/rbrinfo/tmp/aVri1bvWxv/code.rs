fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Err(Error::invalid_type(Unexpected::Option, &self))
    }
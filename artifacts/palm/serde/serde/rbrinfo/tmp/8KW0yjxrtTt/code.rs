fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        str::from_utf8(v)
            .map(From::from)
            .map_err(|_| Error::invalid_value(Unexpected::Bytes(v), &self))
    }
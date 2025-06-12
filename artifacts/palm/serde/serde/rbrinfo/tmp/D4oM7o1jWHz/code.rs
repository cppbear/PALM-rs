fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        str::from_utf8(v)
            .map(AsRef::as_ref)
            .map_err(|_| Error::invalid_value(Unexpected::Bytes(v), &self))
    }
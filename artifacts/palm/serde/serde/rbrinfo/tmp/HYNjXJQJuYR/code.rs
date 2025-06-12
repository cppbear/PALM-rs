fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        CString::new(v).map_err(Error::custom)
    }
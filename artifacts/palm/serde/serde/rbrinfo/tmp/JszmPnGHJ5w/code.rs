fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        CString::new(v).map_err(Error::custom)
    }
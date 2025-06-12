fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let _ = deserializer;
        Err(Error::invalid_type(Unexpected::NewtypeStruct, &self))
    }
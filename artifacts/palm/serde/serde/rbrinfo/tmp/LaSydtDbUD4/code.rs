fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        tri!(serializer.serialize_tuple(0)).end()
    }
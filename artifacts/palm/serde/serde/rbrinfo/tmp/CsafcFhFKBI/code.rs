fn serialize_some<T>(self, _value: &T) -> fmt::Result
    where
        T: ?Sized + Serialize,
    {
        Err(fmt::Error)
    }
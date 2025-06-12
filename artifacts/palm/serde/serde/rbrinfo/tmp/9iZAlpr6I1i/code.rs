fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> fmt::Result
    where
        T: ?Sized + Serialize,
    {
        Serialize::serialize(value, self)
    }
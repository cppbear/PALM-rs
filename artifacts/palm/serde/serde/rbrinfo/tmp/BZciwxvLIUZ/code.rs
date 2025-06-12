fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> fmt::Result
    where
        T: ?Sized + Serialize,
    {
        Err(fmt::Error)
    }
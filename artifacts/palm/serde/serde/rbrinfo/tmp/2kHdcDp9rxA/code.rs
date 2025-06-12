fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, fmt::Error> {
        Err(fmt::Error)
    }
fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, fmt::Error> {
        Err(fmt::Error)
    }
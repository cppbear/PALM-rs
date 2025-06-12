fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Self::bad_type(Unsupported::Tuple))
    }
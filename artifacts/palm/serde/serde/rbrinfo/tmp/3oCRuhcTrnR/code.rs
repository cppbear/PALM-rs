fn serialize_tuple(self, _: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(self.bad_type(Unsupported::Tuple))
    }
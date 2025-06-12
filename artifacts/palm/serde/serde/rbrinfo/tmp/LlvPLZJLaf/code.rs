fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, fmt::Error> {
        Err(fmt::Error)
    }
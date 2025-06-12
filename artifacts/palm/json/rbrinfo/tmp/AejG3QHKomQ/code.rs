fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(SerializeMap::Map {
            map: Map::with_capacity(len.unwrap_or(0)),
            next_key: None,
        })
    }
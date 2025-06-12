fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        let mut map = tri!(self.delegate.serialize_map(len.map(|len| len + 1)));
        tri!(map.serialize_entry(self.tag, self.variant_name));
        Ok(map)
    }
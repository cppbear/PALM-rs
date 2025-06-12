fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        let mut map = tri!(self.delegate.serialize_map(Some(1)));
        tri!(map.serialize_entry(self.tag, self.variant_name));
        map.end()
    }
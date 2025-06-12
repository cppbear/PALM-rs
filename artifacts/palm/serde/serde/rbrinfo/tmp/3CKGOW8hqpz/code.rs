fn serialize_unit_variant(
        self,
        _: &'static str,
        _: u32,
        inner_variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        let mut map = tri!(self.delegate.serialize_map(Some(2)));
        tri!(map.serialize_entry(self.tag, self.variant_name));
        tri!(map.serialize_entry(inner_variant, &()));
        map.end()
    }
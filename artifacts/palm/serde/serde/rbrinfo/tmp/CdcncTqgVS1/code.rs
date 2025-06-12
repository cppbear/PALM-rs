fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        inner_variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        let mut map = tri!(self.delegate.serialize_map(Some(2)));
        tri!(map.serialize_entry(self.tag, self.variant_name));
        tri!(map.serialize_key(inner_variant));
        Ok(SerializeStructVariantAsMapValue::new(
            map,
            inner_variant,
            len,
        ))
    }
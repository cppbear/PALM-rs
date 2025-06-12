fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        inner_variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        let mut map = tri!(self.delegate.serialize_map(Some(2)));
        tri!(map.serialize_entry(self.tag, self.variant_name));
        tri!(map.serialize_key(inner_variant));
        Ok(SerializeTupleVariantAsMapValue::new(
            map,
            inner_variant,
            len,
        ))
    }
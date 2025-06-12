fn serialize_struct_variant(
        self,
        _: &'static str,
        _: u32,
        inner_variant: &'static str,
        _: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        tri!(self.0.serialize_key(inner_variant));
        Ok(FlatMapSerializeStructVariantAsMapValue::new(
            self.0,
            inner_variant,
        ))
    }
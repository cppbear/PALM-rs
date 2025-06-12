fn serialize_tuple_variant(
        self,
        _: &'static str,
        _: u32,
        variant: &'static str,
        _: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        tri!(self.0.serialize_key(variant));
        Ok(FlatMapSerializeTupleVariantAsMapValue::new(self.0))
    }
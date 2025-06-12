fn serialize_tuple_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleVariant, E> {
            Ok(SerializeTupleVariant {
                name,
                variant_index,
                variant,
                fields: Vec::with_capacity(len),
                error: PhantomData,
            })
        }
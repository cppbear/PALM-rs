fn serialize_struct_variant(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStructVariant, E> {
            Ok(SerializeStructVariant {
                name,
                variant_index,
                variant,
                fields: Vec::with_capacity(len),
                error: PhantomData,
            })
        }
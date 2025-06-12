fn serialize_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeStruct, E> {
            Ok(SerializeStruct {
                name,
                fields: Vec::with_capacity(len),
                error: PhantomData,
            })
        }
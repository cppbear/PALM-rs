fn serialize_tuple_struct(
            self,
            name: &'static str,
            len: usize,
        ) -> Result<Self::SerializeTupleStruct, E> {
            Ok(SerializeTupleStruct {
                name,
                fields: Vec::with_capacity(len),
                error: PhantomData,
            })
        }
fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, E> {
            Ok(SerializeTuple {
                elements: Vec::with_capacity(len),
                error: PhantomData,
            })
        }
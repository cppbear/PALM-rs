fn new(slice: &'de [Value]) -> Self {
        SeqRefDeserializer { iter: slice.iter() }
    }
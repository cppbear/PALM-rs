pub fn new(iter: I) -> Self {
        MapDeserializer {
            iter: iter.fuse(),
            value: None,
            count: 0,
            lifetime: PhantomData,
            error: PhantomData,
        }
    }
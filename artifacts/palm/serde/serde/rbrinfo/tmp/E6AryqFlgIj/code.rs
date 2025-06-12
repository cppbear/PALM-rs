pub fn new(iter: I) -> Self {
        SeqDeserializer {
            iter: iter.fuse(),
            count: 0,
            marker: PhantomData,
        }
    }
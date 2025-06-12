fn new(de: &'a mut Deserializer<R>) -> Self {
        SeqAccess { de, first: true }
    }
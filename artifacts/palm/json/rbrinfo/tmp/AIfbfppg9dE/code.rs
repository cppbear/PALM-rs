fn new(de: &'a mut Deserializer<R>) -> Self {
        MapAccess { de, first: true }
    }
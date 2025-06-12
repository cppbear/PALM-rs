fn new(de: &'a mut Deserializer<R>) -> Self {
        VariantAccess { de }
    }
fn new(de: &'a mut Deserializer<R>) -> Self {
        UnitVariantAccess { de }
    }
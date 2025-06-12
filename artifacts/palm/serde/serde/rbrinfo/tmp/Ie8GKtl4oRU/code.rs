fn new(map: &'a mut M) -> Self {
        FlatMapSerializeTupleVariantAsMapValue {
            map,
            fields: Vec::new(),
        }
    }
fn new(map: &'a mut M, name: &'static str) -> FlatMapSerializeStructVariantAsMapValue<'a, M> {
        FlatMapSerializeStructVariantAsMapValue {
            map,
            name,
            fields: Vec::new(),
        }
    }
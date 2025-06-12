pub fn new(map: M, name: &'static str, len: usize) -> Self {
            SerializeTupleVariantAsMapValue {
                map,
                name,
                fields: Vec::with_capacity(len),
            }
        }
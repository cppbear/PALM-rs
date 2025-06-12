pub fn new(map: M, name: &'static str, len: usize) -> Self {
            SerializeStructVariantAsMapValue {
                map,
                name,
                fields: Vec::with_capacity(len),
            }
        }
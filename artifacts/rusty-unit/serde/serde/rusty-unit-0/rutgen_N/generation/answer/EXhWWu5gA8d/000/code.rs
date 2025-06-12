// Answer 0

#[derive(Debug)]
struct SerializeTupleVariantAsMapValue<M> {
    map: M,
    name: &'static str,
    fields: Vec<String>,
}

impl<M> SerializeTupleVariantAsMapValue<M> {
    pub fn new(map: M, name: &'static str, len: usize) -> Self {
        SerializeTupleVariantAsMapValue {
            map,
            name,
            fields: Vec::with_capacity(len),
        }
    }
}

#[test]
fn test_serialize_tuple_variant_as_map_value_creation() {
    let map = std::collections::HashMap::new();
    let name = "test_variant";
    let len = 5;

    let instance: SerializeTupleVariantAsMapValue<_> =
        SerializeTupleVariantAsMapValue::new(map, name, len);

    assert_eq!(instance.name, name);
    assert_eq!(instance.fields.capacity(), len);
}

#[test]
fn test_serialize_tuple_variant_as_map_value_empty() {
    let map = std::collections::HashMap::new();
    let name = "empty_variant";
    let len = 0;

    let instance: SerializeTupleVariantAsMapValue<_> =
        SerializeTupleVariantAsMapValue::new(map, name, len);

    assert_eq!(instance.name, name);
    assert_eq!(instance.fields.capacity(), len);
}


// Answer 0

#[derive(Debug)]
struct FlatMapSerializeTupleVariantAsMapValue<'a, M> {
    map: &'a mut M,
    fields: Vec<String>,
}

impl<'a, M> FlatMapSerializeTupleVariantAsMapValue<'a, M> {
    fn new(map: &'a mut M) -> Self {
        FlatMapSerializeTupleVariantAsMapValue {
            map,
            fields: Vec::new(),
        }
    }
}

#[test]
fn test_new_creates_instance_with_provided_map() {
    let mut my_map = std::collections::HashMap::new();
    let instance = FlatMapSerializeTupleVariantAsMapValue::new(&mut my_map);
    
    assert_eq!(instance.map, &mut my_map);
    assert_eq!(instance.fields.len(), 0);
}

#[test]
fn test_new_creates_instance_with_empty_fields() {
    let mut my_map = std::collections::HashMap::new();
    let instance = FlatMapSerializeTupleVariantAsMapValue::new(&mut my_map);

    assert!(instance.fields.is_empty());
}


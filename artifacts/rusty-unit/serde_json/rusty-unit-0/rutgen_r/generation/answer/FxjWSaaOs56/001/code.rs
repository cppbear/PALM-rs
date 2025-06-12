// Answer 0

#[derive(Debug)]
struct SerializeTupleVariant {
    name: String,
    vec: Vec<i32>,
}

struct Serializer;

impl Serializer {
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<SerializeTupleVariant, &'static str> {
        Ok(SerializeTupleVariant {
            name: String::from(variant),
            vec: Vec::with_capacity(len),
        })
    }
}

#[test]
fn test_serialize_tuple_variant_empty() {
    let serializer = Serializer;
    let result = serializer.serialize_tuple_variant("Test", 0, "VariantName", 0);
    assert!(result.is_ok());
    let serialized = result.unwrap();
    assert_eq!(serialized.name, "VariantName");
    assert_eq!(serialized.vec.capacity(), 0);
}

#[test]
fn test_serialize_tuple_variant_non_empty() {
    let serializer = Serializer;
    let result = serializer.serialize_tuple_variant("Test", 0, "AnotherVariant", 10);
    assert!(result.is_ok());
    let serialized = result.unwrap();
    assert_eq!(serialized.name, "AnotherVariant");
    assert_eq!(serialized.vec.capacity(), 10);
}

#[test]
fn test_serialize_tuple_variant_with_large_capacity() {
    let serializer = Serializer;
    let large_capacity = usize::MAX; // edge case for capacity
    let result = serializer.serialize_tuple_variant("Test", 0, "MaxCapacityVariant", large_capacity);
    assert!(result.is_ok());
    let serialized = result.unwrap();
    assert_eq!(serialized.name, "MaxCapacityVariant");
    assert_eq!(serialized.vec.capacity(), large_capacity);
}


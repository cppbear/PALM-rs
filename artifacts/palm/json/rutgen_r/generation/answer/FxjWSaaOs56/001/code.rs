// Answer 0

#[derive(Debug)]
struct SerializeTupleVariant {
    name: String,
    vec: Vec<u8>,
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
fn test_serialize_tuple_variant_non_empty() {
    let serializer = Serializer;
    let result = serializer.serialize_tuple_variant("Test", 0, "VariantA", 5);
    assert!(result.is_ok());
    let tuple_variant = result.unwrap();
    assert_eq!(tuple_variant.name, "VariantA");
    assert_eq!(tuple_variant.vec.capacity(), 5);
}

#[test]
fn test_serialize_tuple_variant_empty() {
    let serializer = Serializer;
    let result = serializer.serialize_tuple_variant("Test", 1, "VariantB", 0);
    assert!(result.is_ok());
    let tuple_variant = result.unwrap();
    assert_eq!(tuple_variant.name, "VariantB");
    assert_eq!(tuple_variant.vec.capacity(), 0);
}

#[test]
fn test_serialize_tuple_variant_large_capacity() {
    let serializer = Serializer;
    let large_len = 1000000;
    let result = serializer.serialize_tuple_variant("Test", 2, "VariantC", large_len);
    assert!(result.is_ok());
    let tuple_variant = result.unwrap();
    assert_eq!(tuple_variant.name, "VariantC");
    assert_eq!(tuple_variant.vec.capacity(), large_len);
}


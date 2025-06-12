// Answer 0

#[test]
fn test_serialize_tuple_variant_as_map_value_new() {
    // Test with a simple map
    let simple_map = std::collections::HashMap::<String, i32>::new();
    let name = "test_variant";
    let len = 5;
    
    let result = SerializeTupleVariantAsMapValue::new(simple_map, name, len);
    assert_eq!(result.name, name);
    assert_eq!(result.fields.capacity(), len);
}

#[test]
fn test_serialize_tuple_variant_as_map_value_new_empty_map() {
    // Test with an empty map
    let empty_map = std::collections::HashMap::<String, i32>::new();
    let name = "empty_variant";
    let len = 0;
    
    let result = SerializeTupleVariantAsMapValue::new(empty_map, name, len);
    assert_eq!(result.name, name);
    assert_eq!(result.fields.capacity(), len);
}

#[test]
fn test_serialize_tuple_variant_as_map_value_new_capacity_one() {
    // Test with capacity of one
    let single_item_map = std::collections::HashMap::<String, i32>::new();
    let name = "single_item_variant";
    let len = 1;
    
    let result = SerializeTupleVariantAsMapValue::new(single_item_map, name, len);
    assert_eq!(result.name, name);
    assert_eq!(result.fields.capacity(), len);
}


// Answer 0

#[test]
fn test_new_with_non_empty_map() {
    struct TestMap {
        data: Vec<(Content, Content)>,
    }

    let map = TestMap { data: vec![(Content::String("key".to_string()), Content::U32(42))] };
    let name = "test_variant";
    let len = 1;

    let tuple_variant = SerializeTupleVariantAsMapValue::new(map, name, len);
    
    assert_eq!(tuple_variant.name, name);
    assert_eq!(tuple_variant.fields.capacity(), len);
}

#[test]
fn test_new_with_empty_map() {
    struct TestMap {
        data: Vec<(Content, Content)>,
    }

    let map = TestMap { data: vec![] };
    let name = "empty_variant";
    let len = 0;

    let tuple_variant = SerializeTupleVariantAsMapValue::new(map, name, len);

    assert_eq!(tuple_variant.name, name);
    assert_eq!(tuple_variant.fields.capacity(), len);
}

#[test]
fn test_new_with_large_capacity() {
    struct TestMap {
        data: Vec<(Content, Content)>,
    }

    let map = TestMap { data: vec![] };
    let name = "large_capacity_variant";
    let len = 100;

    let tuple_variant = SerializeTupleVariantAsMapValue::new(map, name, len);

    assert_eq!(tuple_variant.name, name);
    assert_eq!(tuple_variant.fields.capacity(), len);
}


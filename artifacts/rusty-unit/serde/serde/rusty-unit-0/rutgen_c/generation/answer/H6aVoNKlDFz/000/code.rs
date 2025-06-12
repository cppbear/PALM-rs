// Answer 0

#[test]
fn test_new_with_valid_data() {
    struct MyMap;
    let map = MyMap;
    let name = "test_variant";
    let len = 5;

    let variant = SerializeStructVariantAsMapValue::new(map, name, len);
    assert_eq!(variant.name, name);
    assert_eq!(variant.fields.capacity(), len);
}

#[test]
fn test_new_with_zero_length() {
    struct MyMap;
    let map = MyMap;
    let name = "empty_variant";
    let len = 0;

    let variant = SerializeStructVariantAsMapValue::new(map, name, len);
    assert_eq!(variant.name, name);
    assert_eq!(variant.fields.capacity(), len);
}


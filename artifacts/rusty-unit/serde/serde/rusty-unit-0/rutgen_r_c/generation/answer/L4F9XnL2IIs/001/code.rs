// Answer 0

#[test]
fn test_serialize_unit_variant_success() {
    struct MockError;
    impl ser::Error for MockError {}

    let serializer = ContentSerializer::<MockError> { error: PhantomData };

    let name = "TestEnum";
    let variant_index = 0;
    let variant = "TestVariant";

    match serializer.serialize_unit_variant(name, variant_index, variant) {
        Ok(content) => {
            match content {
                Content::UnitVariant(received_name, received_index, received_variant) => {
                    assert_eq!(received_name, name);
                    assert_eq!(received_index, variant_index);
                    assert_eq!(received_variant, variant);
                },
                _ => panic!("Expected UnitVariant"),
            }
        },
        Err(_) => panic!("Expected Ok, got Err"),
    }
}

#[test]
fn test_serialize_unit_variant_different_input() {
    struct MockError;
    impl ser::Error for MockError {}

    let serializer = ContentSerializer::<MockError> { error: PhantomData };

    let name = "AnotherEnum";
    let variant_index = 1;
    let variant = "AnotherVariant";

    match serializer.serialize_unit_variant(name, variant_index, variant) {
        Ok(content) => {
            match content {
                Content::UnitVariant(received_name, received_index, received_variant) => {
                    assert_eq!(received_name, name);
                    assert_eq!(received_index, variant_index);
                    assert_eq!(received_variant, variant);
                },
                _ => panic!("Expected UnitVariant"),
            }
        },
        Err(_) => panic!("Expected Ok, got Err"),
    }
}


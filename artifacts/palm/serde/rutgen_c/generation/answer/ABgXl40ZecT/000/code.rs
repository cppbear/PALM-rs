// Answer 0

#[test]
fn test_end_function() {
    struct TestError;
    impl ser::Error for TestError {}

    let fields = vec![
        ("field1", Content::U8(42)),
        ("field2", Content::String("test".to_string())),
    ];
    let serialize_struct_variant = SerializeStructVariant {
        name: "TestStruct",
        variant_index: 0,
        variant: "TestVariant",
        fields,
        error: PhantomData::<TestError>,
    };

    let result = serialize_struct_variant.end();
    assert!(result.is_ok());

    let content = result.unwrap();
    match content {
        Content::StructVariant(name, index, variant, fields) => {
            assert_eq!(name, "TestStruct");
            assert_eq!(index, 0);
            assert_eq!(variant, "TestVariant");
            assert_eq!(fields.len(), 2);
        },
        _ => panic!("Unexpected content type"),
    }
}

#[test]
fn test_end_empty_fields() {
    struct TestError;
    impl ser::Error for TestError {}

    let serialize_struct_variant = SerializeStructVariant {
        name: "EmptyStruct",
        variant_index: 1,
        variant: "EmptyVariant",
        fields: Vec::new(),
        error: PhantomData::<TestError>,
    };

    let result = serialize_struct_variant.end();
    assert!(result.is_ok());

    let content = result.unwrap();
    match content {
        Content::StructVariant(name, index, variant, fields) => {
            assert_eq!(name, "EmptyStruct");
            assert_eq!(index, 1);
            assert_eq!(variant, "EmptyVariant");
            assert!(fields.is_empty());
        },
        _ => panic!("Unexpected content type"),
    }
}


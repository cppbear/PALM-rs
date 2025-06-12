// Answer 0

#[test]
fn test_end_success() {
    struct TestError;
    impl ser::Error for TestError {}

    let name = "test_name";
    let variant_index = 1;
    let variant = "test_variant";
    let fields = vec![
        ("field1", Content::U8(10)),
        ("field2", Content::String("value".to_string())),
    ];

    let serialize_struct_variant = SerializeStructVariant {
        name,
        variant_index,
        variant,
        fields,
        error: PhantomData::<TestError>,
    };

    let result = serialize_struct_variant.end();
    assert!(result.is_ok());
    if let Ok(content) = result {
        match content {
            Content::StructVariant(n, vi, v, f) => {
                assert_eq!(n, name);
                assert_eq!(vi, variant_index);
                assert_eq!(v, variant);
                assert_eq!(f, fields);
            },
            _ => panic!("Unexpected content variant"),
        }
    }
}

#[test]
fn test_end_with_empty_fields() {
    struct TestError;
    impl ser::Error for TestError {}

    let name = "empty_fields_name";
    let variant_index = 0;
    let variant = "empty_fields_variant";
    let fields: Vec<(&'static str, Content)> = vec![];

    let serialize_struct_variant = SerializeStructVariant {
        name,
        variant_index,
        variant,
        fields,
        error: PhantomData::<TestError>,
    };

    let result = serialize_struct_variant.end();
    assert!(result.is_ok());
    if let Ok(content) = result {
        match content {
            Content::StructVariant(n, vi, v, f) => {
                assert_eq!(n, name);
                assert_eq!(vi, variant_index);
                assert_eq!(v, variant);
                assert!(f.is_empty());
            },
            _ => panic!("Unexpected content variant"),
        }
    }
}


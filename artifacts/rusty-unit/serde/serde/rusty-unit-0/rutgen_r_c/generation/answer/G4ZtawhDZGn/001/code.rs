// Answer 0

#[test]
fn test_end_returns_ok_with_correct_content() {
    struct TestError;
    impl ser::Error for TestError {}

    let name = "test_name";
    let variant_index = 0;
    let variant = "test_variant";
    let fields = vec![
        Content::U8(42),
        Content::String("Hello".to_string())
    ];

    let serialize_tuple_variant: SerializeTupleVariant<TestError> = SerializeTupleVariant {
        name,
        variant_index,
        variant,
        fields,
        error: std::marker::PhantomData,
    };

    let result = serialize_tuple_variant.end();
    assert!(result.is_ok());

    let content = result.unwrap();
    if let Content::TupleVariant(n, vi, v, f) = content {
        assert_eq!(n, name);
        assert_eq!(vi, variant_index);
        assert_eq!(v, variant);
        assert_eq!(f, fields);
    } else {
        panic!("Unexpected content variant");
    }
}

#[test]
fn test_end_with_empty_fields() {
    struct TestError;
    impl ser::Error for TestError {}

    let name = "empty_fields_name";
    let variant_index = 1;
    let variant = "empty_fields_variant";
    let fields: Vec<Content> = vec![];

    let serialize_tuple_variant: SerializeTupleVariant<TestError> = SerializeTupleVariant {
        name,
        variant_index,
        variant,
        fields,
        error: std::marker::PhantomData,
    };

    let result = serialize_tuple_variant.end();
    assert!(result.is_ok());

    let content = result.unwrap();
    if let Content::TupleVariant(n, vi, v, f) = content {
        assert_eq!(n, name);
        assert_eq!(vi, variant_index);
        assert_eq!(v, variant);
        assert_eq!(f, fields);
    } else {
        panic!("Unexpected content variant");
    }
}


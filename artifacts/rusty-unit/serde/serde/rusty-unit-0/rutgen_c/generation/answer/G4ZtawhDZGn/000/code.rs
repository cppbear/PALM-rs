// Answer 0

#[test]
fn test_end_function() {
    struct DummyError;
    
    impl ser::Error for DummyError {}
    
    let name = "test_variant";
    let variant_index = 0;
    let variant = "TestVariant";
    let fields = vec![Content::U32(42), Content::String("Hello".to_string())];

    let serializer = SerializeTupleVariant {
        name,
        variant_index,
        variant,
        fields,
        error: PhantomData::<DummyError>,
    };

    let result = serializer.end();
    
    assert!(result.is_ok());
    let content = result.unwrap();
    
    match content {
        Content::TupleVariant(n, vi, v, f) => {
            assert_eq!(n, "test_variant");
            assert_eq!(vi, 0);
            assert_eq!(v, "TestVariant");
            assert_eq!(f.len(), 2);
            if let Content::U32(value) = &f[0] {
                assert_eq!(*value, 42);
            } else {
                panic!("Expected U32 content");
            }
            if let Content::String(ref s) = &f[1] {
                assert_eq!(s, "Hello");
            } else {
                panic!("Expected String content");
            }
        }
        _ => panic!("Expected TupleVariant content"),
    }
}

#[test]
#[should_panic]
fn test_end_function_empty_fields() {
    struct DummyError;

    impl ser::Error for DummyError {}

    let name = "test_variant";
    let variant_index = 1;
    let variant = "TestVariant";
    let fields: Vec<Content> = vec![];

    let serializer = SerializeTupleVariant {
        name,
        variant_index,
        variant,
        fields,
        error: PhantomData::<DummyError>,
    };

    let result = serializer.end();

    assert!(result.is_ok());
    
    let content = result.unwrap();
    match content {
        Content::TupleVariant(n, vi, v, f) => {
            assert_eq!(n, "test_variant");
            assert_eq!(vi, 1);
            assert_eq!(v, "TestVariant");
            assert!(f.is_empty());
        }
        _ => panic!("Expected TupleVariant content"),
    }
}


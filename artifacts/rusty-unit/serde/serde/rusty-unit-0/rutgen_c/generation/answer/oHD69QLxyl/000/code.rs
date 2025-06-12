// Answer 0

#[test]
fn test_end_creates_tuple_struct_content() {
    struct TestError;
    impl ser::Error for TestError {}

    let name = "TestStruct";
    let fields = vec![
        Content::Bool(true),
        Content::U32(42),
        Content::String("Hello".to_string()),
    ];

    let serializer: SerializeTupleStruct<TestError> = SerializeTupleStruct {
        name,
        fields,
        error: PhantomData,
    };

    let result = serializer.end();

    assert!(result.is_ok());

    let content = result.unwrap();
    match content {
        Content::TupleStruct(struct_name, struct_fields) => {
            assert_eq!(struct_name, name);
            assert_eq!(struct_fields.len(), 3);
            assert_eq!(struct_fields[0], Content::Bool(true));
            assert_eq!(struct_fields[1], Content::U32(42));
            assert_eq!(struct_fields[2], Content::String("Hello".to_string()));
        }
        _ => panic!("Expected Content::TupleStruct"),
    }
}

#[test]
#[should_panic]
fn test_end_on_empty_fields_panics() {
    struct TestError;
    impl ser::Error for TestError {}

    let name = "EmptyStruct";
    let fields: Vec<Content> = vec![];

    let serializer: SerializeTupleStruct<TestError> = SerializeTupleStruct {
        name,
        fields,
        error: PhantomData,
    };

    let result = serializer.end();

    assert!(result.is_ok());
    let content = result.unwrap();
    if let Content::TupleStruct(struct_name, struct_fields) = content {
        assert_eq!(struct_name, name);
        assert!(struct_fields.is_empty());
    } else {
        panic!("Expected Content::TupleStruct");
    }
}


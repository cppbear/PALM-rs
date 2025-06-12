// Answer 0

#[test]
fn test_end_function() {
    struct TestError;

    impl std::fmt::Debug for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "TestError")
        }
    }

    impl ser::Error for TestError {}

    let name = "test_struct";
    let fields = vec![
        ("field1", Content::U32(42)),
        ("field2", Content::String("example".to_string())),
    ];

    let serialize_struct = SerializeStruct {
        name,
        fields,
        error: PhantomData::<TestError>,
    };

    let result = serialize_struct.end();
    assert!(result.is_ok());
    if let Ok(content) = result {
        match content {
            Content::Struct(s_name, s_fields) => {
                assert_eq!(s_name, name);
                assert_eq!(s_fields.len(), fields.len());
                for (key, value) in fields {
                    assert!(s_fields.contains(&(key, value)));
                }
            }
            _ => panic!("Expected Content to be a Struct"),
        }
    }
}

#[test]
fn test_end_function_empty_fields() {
    struct TestError;

    impl std::fmt::Debug for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "TestError")
        }
    }

    impl ser::Error for TestError {}

    let name = "empty_struct";
    let fields: Vec<(&'static str, Content)> = vec![];

    let serialize_struct = SerializeStruct {
        name,
        fields,
        error: PhantomData::<TestError>,
    };

    let result = serialize_struct.end();
    assert!(result.is_ok());
    if let Ok(content) = result {
        match content {
            Content::Struct(s_name, s_fields) => {
                assert_eq!(s_name, name);
                assert!(s_fields.is_empty());
            }
            _ => panic!("Expected Content to be a Struct"),
        }
    }
}


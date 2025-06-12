// Answer 0

#[test]
fn test_end() {
    struct MockError;

    impl std::fmt::Debug for MockError {
        fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
            Ok(())
        }
    }

    impl serde::Error for MockError {
        fn custom<T>(_msg: T) -> Self {
            MockError
        }
    }

    let name = "test_struct";
    let fields = vec![
        Content::U8(42),
        Content::String("Hello".to_string()),
        Content::Bool(true),
    ];

    let serializer: SerializeTupleStruct<MockError> = SerializeTupleStruct {
        name,
        fields,
        error: std::marker::PhantomData,
    };

    let result = serializer.end();
    assert_eq!(result, Ok(Content::TupleStruct(name, fields)));
}


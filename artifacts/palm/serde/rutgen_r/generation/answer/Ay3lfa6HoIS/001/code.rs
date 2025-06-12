// Answer 0

#[test]
fn test_serialize_i16_should_return_error() {
    struct MockSerializer;

    impl MockSerializer {
        fn bad_type(&self, _: Unsupported) -> Result<(), String> {
            Err(String::from("Unsupported type"))
        }
    }

    enum Unsupported {
        Integer,
    }

    let serializer = MockSerializer;
    let result = serializer.serialize_i16(42);

    match result {
        Err(error) => assert_eq!(error, "Unsupported type"),
        _ => panic!("Expected an Err but got a different result"),
    }
}


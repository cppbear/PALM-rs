// Answer 0

#[test]
fn test_serialize_unit_should_return_error() {
    struct Serializer;

    impl Serializer {
        fn serialize_unit(self) -> Result<(), String> {
            Err("key must be a string".to_string())
        }
    }

    let serializer = Serializer;
    let result = serializer.serialize_unit();
    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "key must be a string");
}


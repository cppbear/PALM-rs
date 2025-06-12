// Answer 0

#[test]
fn test_serialize_map_err() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_map(self, _len: Option<usize>) -> Result<(), String> {
            Err("key must be a string".to_string())
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_map(None);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "key must be a string".to_string());
}


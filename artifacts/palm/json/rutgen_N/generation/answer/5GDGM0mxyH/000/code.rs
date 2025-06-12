// Answer 0

#[test]
fn test_serialize_map_returns_error() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_map(self, _len: Option<usize>) -> Result<TestSerializeMap> {
            Err(key_must_be_a_string())
        }
    }

    struct TestSerializeMap;

    let serializer = TestSerializer;
    let result = serializer.serialize_map(None);
    assert!(result.is_err());
}


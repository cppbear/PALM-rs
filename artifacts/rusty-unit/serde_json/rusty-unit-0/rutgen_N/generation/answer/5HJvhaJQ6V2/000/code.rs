// Answer 0

#[test]
fn test_serialize_bytes_error() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_bytes(self, value: &[u8]) -> Result<()> {
            Err(key_must_be_a_string())
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_bytes(b"test_bytes");
    assert!(result.is_err());
}


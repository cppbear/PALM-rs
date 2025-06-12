// Answer 0

#[test]
fn test_serialize_bytes_err() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(&self, _: Unsupported) -> Result<(), &'static str> {
            Err("Unsupported ByteArray")
        }
    }

    let serializer = TestSerializer;
    let bytes: &[u8] = b"test bytes";
    let result = serializer.serialize_bytes(bytes);
    assert!(result.is_err());
}


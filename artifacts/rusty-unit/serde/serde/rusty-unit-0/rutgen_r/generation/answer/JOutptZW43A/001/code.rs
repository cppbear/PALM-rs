// Answer 0

#[test]
fn test_serialize_str_returns_err_for_unsupported_string() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(&self, _: Unsupported) -> Result<(), String> {
            Err("Unsupported type".to_string())
        }
    }

    let serializer = TestSerializer;

    let result = serializer.serialize_str("test string");

    assert_eq!(result, Err("Unsupported type".to_string()));
}


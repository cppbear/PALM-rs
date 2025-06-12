// Answer 0

#[test]
fn test_serialize_f64() {
    struct DummySerializer;

    impl DummySerializer {
        fn bad_type(&self, _: Unsupported) -> Result<(), String> {
            Err("Unsupported type".to_string())
        }
    }

    let serializer = DummySerializer;
    let result = serializer.serialize_f64(5.0);
    assert_eq!(result, Err("Unsupported type".to_string()));
}


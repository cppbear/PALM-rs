// Answer 0

#[test]
fn test_serialize_i64_err() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(&self, _: Unsupported) -> Result<(), &'static str> {
            Err("bad type")
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_i64(42);
    assert_eq!(result, Err("bad type"));
}


// Answer 0

#[test]
fn test_serialize_unit_returns_error() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_unit(self) -> Result<(), fmt::Error> {
            Err(fmt::Error)
        }
    }

    let serializer = TestSerializer;

    let result = serializer.serialize_unit();
    assert!(result.is_err());
    assert_eq!(result, Err(fmt::Error));
}


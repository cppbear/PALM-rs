// Answer 0

#[test]
fn test_serialize_unit_success() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_unit(self) -> Result<(), ()> {
            Ok(())
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_unit();
    assert_eq!(result, Ok(()));
}


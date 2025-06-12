// Answer 0

#[test]
fn test_serialize_none() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_unit(&self) -> Result<()> {
            Ok(())
        }

        fn serialize_none(self) -> Result<()> {
            self.serialize_unit()
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_none();
    assert!(result.is_ok());
}


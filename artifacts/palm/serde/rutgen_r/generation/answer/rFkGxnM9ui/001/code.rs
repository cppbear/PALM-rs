// Answer 0

#[test]
fn test_serialize_u8_returns_err() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(&self, _: Unsupported) -> Result<(), ()> {
            Err(())
        }
    }

    struct Unsupported;

    let serializer = TestSerializer;
    let result = serializer.serialize_u8(1);
    assert!(result.is_err());
}

impl TestSerializer {
    fn serialize_u8(self, _: u8) -> Result<(), ()> {
        Err(self.bad_type(Unsupported))
    }
}


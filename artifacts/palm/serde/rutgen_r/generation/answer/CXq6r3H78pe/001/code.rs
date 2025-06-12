// Answer 0

#[test]
fn test_serialize_u16_err_case() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(&self, _: Unsupported) -> Result<(), ()> {
            Err(())
        }

        fn serialize_u16(self, _: u16) -> Result<(), ()> {
            Err(self.bad_type(Unsupported::Integer))
        }
    }

    enum Unsupported {
        Integer,
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_u16(42_u16);
    assert_eq!(result, Err(()));
}


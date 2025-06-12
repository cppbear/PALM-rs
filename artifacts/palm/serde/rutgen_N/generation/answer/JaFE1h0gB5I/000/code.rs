// Answer 0

#[test]
fn test_serialize_u32_error() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(&self, _: Unsupported) -> ! {
            panic!("bad type error")
        }
    }

    enum Unsupported {
        Integer,
    }

    let serializer = TestSerializer;
    let result: Result<(), ()> = serializer.serialize_u32(42).map_err(|_| ());

    assert!(result.is_err());
}


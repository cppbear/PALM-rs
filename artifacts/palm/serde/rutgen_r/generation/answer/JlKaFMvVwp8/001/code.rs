// Answer 0

#[test]
fn test_serialize_u64_returns_error() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(_: Unsupported) -> ErrType {
            ErrType::new("Bad type")
        }

        fn serialize_u64(self, _: u64) -> Result<(), ErrType> {
            Err(self.bad_type(Unsupported::Integer))
        }
    }

    enum Unsupported {
        Integer,
    }

    struct ErrType {
        message: &'static str,
    }

    impl ErrType {
        fn new(message: &'static str) -> Self {
            ErrType { message }
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_u64(42);
    assert!(result.is_err());
    if let Err(err) = result {
        assert_eq!(err.message, "Bad type");
    }
}


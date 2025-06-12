// Answer 0

#[test]
fn test_serialize_i32_error() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(&self, _: Unsupported) -> Result<(), String> {
            Err("Unsupported type".to_string())
        }

        fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
            Err(self.bad_type(Unsupported::Integer))
        }
    }

    struct Unsupported;
    impl Unsupported {
        const Integer: Self = Unsupported;
    }

    type ResultType = Result<(), String>;

    let serializer = TestSerializer;
    let result: ResultType = serializer.serialize_i32(42);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Unsupported type".to_string());
}


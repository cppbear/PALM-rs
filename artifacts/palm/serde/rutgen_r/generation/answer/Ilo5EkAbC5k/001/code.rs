// Answer 0

#[test]
fn test_serialize_u32_should_return_err() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(_: Unsupported) -> String {
            "Unsupported type".to_string()
        }
    }
    
    enum Unsupported {
        Integer,
    }

    let serializer = TestSerializer;
    let result: Result<String, String> = serializer.serialize_u32(42);
    assert_eq!(result, Err("Unsupported type".to_string()));
}

impl TestSerializer {
    fn serialize_u32(self, _: u32) -> Result<String, String> {
        Err(self.bad_type(Unsupported::Integer))
    }
}


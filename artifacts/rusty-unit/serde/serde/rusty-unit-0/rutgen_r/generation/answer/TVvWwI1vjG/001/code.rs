// Answer 0

#[test]
fn test_serialize_str_err() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(_: Unsupported) -> String {
            "bad type".to_string()
        }

        fn serialize_str(self, _: &str) -> Result<String, String> {
            Err(Self::bad_type(Unsupported::String))
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_str("test string");
    
    assert_eq!(result, Err("bad type".to_string()));
}

#[test]
fn test_serialize_empty_str_err() {
    struct TestSerializer;

    impl TestSerializer {
        fn bad_type(_: Unsupported) -> String {
            "bad type".to_string()
        }

        fn serialize_str(self, _: &str) -> Result<String, String> {
            Err(Self::bad_type(Unsupported::String))
        }
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_str("");
    
    assert_eq!(result, Err("bad type".to_string()));
}


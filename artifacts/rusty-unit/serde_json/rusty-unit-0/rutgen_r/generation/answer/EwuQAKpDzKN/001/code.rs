// Answer 0

#[test]
fn test_serialize_map_key_must_be_a_string() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_map(self, _len: Option<usize>) -> Result<(), String> {
            Err(key_must_be_a_string())
        }
    }
    
    fn key_must_be_a_string() -> String {
        "keys must be strings".to_string()
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_map(None);
    assert_eq!(result, Err("keys must be strings".to_string()));
}

#[test]
fn test_serialize_map_with_length_key_must_be_a_string() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_map(self, _len: Option<usize>) -> Result<(), String> {
            Err(key_must_be_a_string())
        }
    }
    
    fn key_must_be_a_string() -> String {
        "keys must be strings".to_string()
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_map(Some(10));
    assert_eq!(result, Err("keys must be strings".to_string()));
}


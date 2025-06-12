// Answer 0

#[test]
fn test_serialize_map_err_key_must_be_a_string() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_map(self, _len: Option<usize>) -> Result<(), String> {
            Err(key_must_be_a_string())
        }
    }

    fn key_must_be_a_string() -> String {
        "Key must be a string".to_string()
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_map(Some(3));

    assert_eq!(result, Err("Key must be a string".to_string()));
}

#[test]
fn test_serialize_map_err_with_different_len() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_map(self, _len: Option<usize>) -> Result<(), String> {
            Err(key_must_be_a_string())
        }
    }

    fn key_must_be_a_string() -> String {
        "Key must be a string".to_string()
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_map(Some(0));

    assert_eq!(result, Err("Key must be a string".to_string()));
}

#[test]
fn test_serialize_map_err_with_none_len() {
    struct TestSerializer;

    impl TestSerializer {
        fn serialize_map(self, _len: Option<usize>) -> Result<(), String> {
            Err(key_must_be_a_string())
        }
    }

    fn key_must_be_a_string() -> String {
        "Key must be a string".to_string()
    }

    let serializer = TestSerializer;
    let result = serializer.serialize_map(None);

    assert_eq!(result, Err("Key must be a string".to_string()));
}


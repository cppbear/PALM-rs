// Answer 0

#[test]
fn test_serialize_map_err_key_must_be_a_string() {
    struct MySerializer;

    impl MySerializer {
        fn serialize_map(self, _len: Option<usize>) -> Result<(), &'static str> {
            Err(key_must_be_a_string())
        }
    }

    fn key_must_be_a_string() -> &'static str {
        "Key must be a string"
    }

    let serializer = MySerializer;
    let result = serializer.serialize_map(None);
    assert_eq!(result, Err("Key must be a string"));
}

#[test]
fn test_serialize_map_err_key_must_be_a_string_with_length() {
    struct MySerializer;

    impl MySerializer {
        fn serialize_map(self, _len: Option<usize>) -> Result<(), &'static str> {
            Err(key_must_be_a_string())
        }
    }

    fn key_must_be_a_string() -> &'static str {
        "Key must be a string"
    }

    let serializer = MySerializer;
    let result = serializer.serialize_map(Some(5));
    assert_eq!(result, Err("Key must be a string"));
}


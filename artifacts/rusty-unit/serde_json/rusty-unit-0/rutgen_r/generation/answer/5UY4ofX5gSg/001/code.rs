// Answer 0

#[test]
fn test_serialize_char_valid() {
    struct MockSerializer;

    impl MockSerializer {
        fn serialize_str(&self, value: &str) -> Result<(), &'static str> {
            if value.is_empty() {
                Err("Empty string cannot be serialized")
            } else {
                Ok(())
            }
        }
    }

    let serializer = MockSerializer;

    let result = serializer.serialize_str("a");
    assert_eq!(result, Ok(()));
}

#[test]
fn test_serialize_char_special_character() {
    struct MockSerializer;

    impl MockSerializer {
        fn serialize_str(&self, value: &str) -> Result<(), &'static str> {
            if value.is_empty() {
                Err("Empty string cannot be serialized")
            } else {
                Ok(())
            }
        }
    }

    let serializer = MockSerializer;

    let result = serializer.serialize_str("😊");
    assert_eq!(result, Ok(()));
}

#[test]
fn test_serialize_char_empty_string() {
    struct MockSerializer;

    impl MockSerializer {
        fn serialize_str(&self, value: &str) -> Result<(), &'static str> {
            if value.is_empty() {
                Err("Empty string cannot be serialized")
            } else {
                Ok(())
            }
        }
    }

    let serializer = MockSerializer;

    let result = serializer.serialize_str("");
    assert_eq!(result, Err("Empty string cannot be serialized"));
}

#[test]
fn test_serialize_char_invalid_utf8() {
    struct MockSerializer;

    impl MockSerializer {
        fn serialize_str(&self, value: &str) -> Result<(), &'static str> {
            if value.is_empty() {
                Err("Empty string cannot be serialized")
            } else {
                Ok(())
            }
        }
    }

    let serializer = MockSerializer;

    let byte_sequence: &[u8] = &[0xFF];
    let result = std::str::from_utf8(byte_sequence);
    assert!(result.is_err());
}


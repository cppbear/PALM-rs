// Answer 0

#[test]
fn test_serialize_bytes() {
    struct DummySerializer;

    impl DummySerializer {
        fn serialize_bytes(self, value: &[u8]) -> Result<Content, &'static str> {
            Ok(Content::Bytes(value.to_owned()))
        }
    }

    struct Content {
        data: Vec<u8>,
    }

    impl Content {
        fn Bytes(data: Vec<u8>) -> Self {
            Content { data }
        }
    }

    let serializer = DummySerializer;
    let input: &[u8] = b"test data";
    let result = serializer.serialize_bytes(input);

    assert!(result.is_ok());
    if let Ok(content) = result {
        assert_eq!(content.data, input.to_owned());
    }
}

#[test]
fn test_serialize_empty_bytes() {
    struct DummySerializer;

    impl DummySerializer {
        fn serialize_bytes(self, value: &[u8]) -> Result<Content, &'static str> {
            Ok(Content::Bytes(value.to_owned()))
        }
    }

    struct Content {
        data: Vec<u8>,
    }

    impl Content {
        fn Bytes(data: Vec<u8>) -> Self {
            Content { data }
        }
    }

    let serializer = DummySerializer;
    let input: &[u8] = b"";
    let result = serializer.serialize_bytes(input);

    assert!(result.is_ok());
    if let Ok(content) = result {
        assert_eq!(content.data, input.to_owned());
    }
}


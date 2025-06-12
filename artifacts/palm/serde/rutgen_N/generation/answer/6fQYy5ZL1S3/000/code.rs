// Answer 0

#[test]
fn test_serialize_u64() {
    struct SerdeMock;

    impl SerdeMock {
        fn serialize_u64(self, v: u64) -> Result<Content, &'static str> {
            Ok(Content::U64(v))
        }
    }

    enum Content {
        U64(u64),
    }

    let serializer = SerdeMock;
    let value: u64 = 42;
    let result = serializer.serialize_u64(value);
    
    assert!(result.is_ok());
    if let Ok(content) = result {
        match content {
            Content::U64(v) => assert_eq!(v, value),
        }
    }
}


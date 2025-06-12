// Answer 0

#[test]
fn test_serialize_i32() {
    struct MockSerializer;

    impl MockSerializer {
        fn serialize_i32(self, v: i32) -> Result<Content, String> {
            Ok(Content::I32(v))
        }
    }

    #[derive(Debug, PartialEq)]
    enum Content {
        I32(i32),
    }

    let serializer = MockSerializer;
    let result = serializer.serialize_i32(42);
    assert_eq!(result, Ok(Content::I32(42)));

    let negative_result = serializer.serialize_i32(-10);
    assert_eq!(negative_result, Ok(Content::I32(-10)));
}


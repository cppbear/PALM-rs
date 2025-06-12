// Answer 0

#[test]
fn test_visit_bytes_content() {
    struct MockError;

    impl serde::de::Error for MockError {
        fn custom<T>(_: T) -> Self {
            MockError
        }
    }

    struct DummyVisitor {
        tag: String,
        content: String,
    }

    impl DummyVisitor {
        fn visit_bytes<E>(self, field: &[u8]) -> Result<TagContentOtherField, E>
        where
            E: de::Error,
        {
            if field == self.tag.as_bytes() {
                Ok(TagContentOtherField::Tag)
            } else if field == self.content.as_bytes() {
                Ok(TagContentOtherField::Content)
            } else {
                Ok(TagContentOtherField::Other)
            }
        }
    }

    let visitor = DummyVisitor {
        tag: String::from("tag"),
        content: String::from("content"),
    };

    let result = visitor.visit_bytes(b"content");
    assert_eq!(result, Ok(TagContentOtherField::Content));
}


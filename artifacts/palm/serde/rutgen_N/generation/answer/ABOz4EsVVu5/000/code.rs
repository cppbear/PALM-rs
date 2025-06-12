// Answer 0

#[derive(Debug)]
struct ContentRef;

struct ContentRefDeserializer {
    content: ContentRef,
}

impl ContentRefDeserializer {
    fn new(content: ContentRef) -> Self {
        ContentRefDeserializer { content }
    }
}

impl ContentRef {
    fn into_deserializer(self) -> ContentRefDeserializer {
        ContentRefDeserializer::new(self)
    }
}

#[test]
fn test_into_deserializer() {
    let content = ContentRef;
    let deserializer = content.into_deserializer();
    assert!(matches!(deserializer, ContentRefDeserializer { .. }));
}


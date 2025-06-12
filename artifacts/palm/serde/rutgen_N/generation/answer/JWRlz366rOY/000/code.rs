// Answer 0

#[test]
fn test_new() {
    struct Content<'de> {
        data: &'de str,
    }

    struct ContentDeserializer<'de> {
        content: Content<'de>,
        err: std::marker::PhantomData<()>,
    }

    let content = Content { data: "test data" };
    let deserializer = new(content);
    
    assert_eq!(deserializer.content.data, "test data");
}


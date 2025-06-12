// Answer 0

#[derive(Debug)]
struct Content<'de> {
    data: &'de str,
}

struct ContentRefDeserializer<'a, 'de> {
    content: &'a Content<'de>,
    err: std::marker::PhantomData<&'a ()>,
}

#[test]
fn test_new() {
    let content = Content { data: "test data" };
    let deserializer = new(&content);
    assert_eq!(deserializer.content.data, "test data");
}

#[test]
fn test_new_empty_content() {
    let content = Content { data: "" };
    let deserializer = new(&content);
    assert_eq!(deserializer.content.data, "");
}


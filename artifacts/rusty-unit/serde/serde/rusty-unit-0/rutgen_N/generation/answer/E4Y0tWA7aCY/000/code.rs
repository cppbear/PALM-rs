// Answer 0

#[derive(Debug)]
struct TestVisitor {
    count: usize,
}

impl<'de> serde::de::SeqAccess<'de> for TestVisitor {
    type Error = serde::de::value::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        if self.count < 3 {
            self.count += 1;
            let value = seed.deserialize(serde::de::value::from_str("test").unwrap())?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug)]
struct ContentVisitor;

impl ContentVisitor {
    fn new() -> Self {
        ContentVisitor
    }

    fn visit_seq<V>(self, visitor: V) -> Result<Vec<String>, V::Error>
    where
        V: serde::de::SeqAccess<'de>,
    {
        let mut result = Vec::new();
        while let Some(value) = visitor.next_element_seed(serde::de::value::StringDeserializer::new())? {
            result.push(value);
        }
        Ok(result)
    }
}

#[derive(Debug)]
struct TagOrContent {
    content: Vec<String>,
}

impl TagOrContent {
    fn Content(values: Vec<String>) -> Self {
        TagOrContent { content: values }
    }
}

#[test]
fn test_visit_seq() {
    let visitor = TestVisitor { count: 0 };
    let content = ContentVisitor::new()
        .visit_seq(visitor)
        .map(TagOrContent::Content)
        .unwrap();

    assert_eq!(content.content.len(), 3);
    assert_eq!(content.content[0], "test");
    assert_eq!(content.content[1], "test");
    assert_eq!(content.content[2], "test");
}


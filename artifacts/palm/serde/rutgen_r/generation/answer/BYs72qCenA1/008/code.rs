// Answer 0

#[derive(Debug)]
struct MockMapAccess {
    keys: Vec<TagOrContent>,
    values: Vec<Content>,
    index: usize,
}

impl MockMapAccess {
    fn new(keys: Vec<TagOrContent>, values: Vec<Content>) -> Self {
        Self { keys, values, index: 0 }
    }
}

impl<'de> MapAccess<'de> for MockMapAccess {
    type Error = de::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<TagOrContent>, Self::Error> 
    where
        K: Visitor<'de>,
    {
        if self.index < self.keys.len() {
            let key = self.keys[self.index].clone();
            self.index += 1;
            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn next_value<V>(&mut self) -> Result<V, Self::Error> 
    where
        V: Deserialize<'de>,
    {
        if self.index <= self.values.len() {
            let value = self.values[self.index - 1].clone();
            Ok(value)
        } else {
            Err(de::Error::custom("no more values"))
        }
    }
}

#[test]
fn test_visit_map_duplicate_tag() {
    let map_access = MockMapAccess::new(
        vec![TagOrContent::Tag, TagOrContent::Tag],
        vec![Content::Str("first".to_string()), Content::Str("second".to_string())],
    );

    let result = visit_map(map_access);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_missing_tag() {
    let map_access = MockMapAccess::new(
        vec![TagOrContent::Content(Content::Str("first".to_string()))],
        vec![Content::Str("second".to_string())],
    );

    let result = visit_map(map_access);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_valid() {
    let map_access = MockMapAccess::new(
        vec![TagOrContent::Tag],
        vec![Content::Str("tag_value".to_string()), Content::Str("content_value".to_string())],
    );

    let result = visit_map(map_access);
    assert!(result.is_ok());
}


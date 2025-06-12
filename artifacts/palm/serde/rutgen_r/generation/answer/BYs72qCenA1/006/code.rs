// Answer 0

#[derive(Debug)]
struct TestMap {
    keys: Vec<TagOrContent>,
    values: Vec<Content>,
    index: usize,
}

impl TestMap {
    fn new(keys: Vec<TagOrContent>, values: Vec<Content>) -> Self {
        TestMap { keys, values, index: 0 }
    }
}

impl MapAccess<'_> for TestMap {
    type Error = de::Error;

    fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent>, Self::Error> {
        if self.index < self.keys.len() {
            let key = self.keys[self.index].clone();
            self.index += 1;
            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn next_value<V>(&mut self) -> Result<V, Self::Error> {
        if self.index <= self.values.len() {
            let value = self.values[self.index - 1].clone();
            Ok(value)
        } else {
            Err(de::Error::custom("No value available"))
        }
    }
}

#[derive(Debug, Clone)]
enum TagOrContent {
    Tag,
    Content(Content),
}

#[derive(Debug, Clone)]
struct Content {
    data: String,
}

#[test]
fn test_visit_map_duplicate_field() {
    let tag_name = "tag_field";
    let tag = Content { data: "tag1".to_string() };
    let value1 = Content { data: "value1".to_string() };
    let value2 = Content { data: "value2".to_string() };

    let map = TestMap::new(
        vec![TagOrContent::Tag, TagOrContent::Tag, TagOrContent::Content(value1.clone()), TagOrContent::Content(value2.clone())],
        vec![tag.clone(), tag.clone(), value1, value2],
    );
    
    let result = visit_map(map);
    
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().to_string(), format!("duplicate field '{}'", tag_name));
}

#[test]
fn test_visit_map_missing_field() {
    let tag_name = "tag_field";
    let value1 = Content { data: "value1".to_string() };

    let map = TestMap::new(
        vec![TagOrContent::Content(value1.clone())],
        vec![value1],
    );

    let result = visit_map(map);
    
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().to_string(), format!("missing field '{}'", tag_name));
}


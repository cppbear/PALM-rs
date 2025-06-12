// Answer 0

#[derive(Debug)]
struct TestVisitor {
    value: Option<Vec<(Content<'static>, Content<'static>)>>,
}

impl<'de> Visitor<'de> for TestVisitor {
    type Value = Vec<(Content<'de>, Content<'de>)>;

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut result = vec![];
        while let Some((key, value)) = map.next_entry()? {
            result.push((key, value));
        }
        Ok(result)
    }

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a map")
    }
}

#[test]
fn test_visit_content_map() {
    let content = vec![
        (Content::String("key1".to_string()), Content::U32(1)),
        (Content::String("key2".to_string()), Content::U32(2)),
    ];
    let visitor = TestVisitor { value: None };
    let result = visit_content_map(content, visitor);
    
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.len(), 2);
    assert!(matches!(value[0].0, Content::String(_)));
    assert!(matches!(value[1].1, Content::U32(_)));
}

#[test]
fn test_visit_content_map_empty() {
    let content: Vec<(Content<'static>, Content<'static>)> = vec![];
    let visitor = TestVisitor { value: None };
    let result = visit_content_map(content, visitor);
    
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.len(), 0);
}


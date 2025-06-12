// Answer 0

fn test_visit_content_map_success() {
    struct TestVisitor {
        value: Option<Vec<Content>>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<Content>;

        fn visit_map<M>(self, _: &mut M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            Ok(self.value.unwrap_or_else(Vec::new))
        }
    }

    let content = vec![
        (Content::String("key1".to_string()), Content::U32(1)),
        (Content::String("key2".to_string()), Content::U32(2)),
    ];

    let visitor = TestVisitor { value: Some(vec![Content::U32(1), Content::U32(2)]) };
    let result: Result<Vec<Content>, Box<dyn std::error::Error>> = visit_content_map(content, visitor);
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value.len(), 2);
}

fn test_visit_content_map_empty() {
    struct TestVisitor {
        value: Option<Vec<Content>>,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<Content>;

        fn visit_map<M>(self, _: &mut M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            Ok(self.value.unwrap_or_else(Vec::new))
        }
    }

    let content: Vec<(Content, Content)> = Vec::new();
    let visitor = TestVisitor { value: Some(vec![]) };
    let result: Result<Vec<Content>, Box<dyn std::error::Error>> = visit_content_map(content, visitor);
    assert!(result.is_ok());
    let value = result.unwrap();
    assert!(value.is_empty());
}


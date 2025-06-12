// Answer 0

#[test]
fn test_visit_map_with_empty_map() {
    struct DummyMapAccess;
    
    impl<'de> MapAccess<'de> for DummyMapAccess {
        type Error = &'static str;

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            Ok(None)
        }

        fn next_value_seed<V>(&mut self, _: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            Err("No value")
        }
    }

    let visitor = TagOrContentVisitor {
        name: "test",
        value: PhantomData,
    };
    let result: Result<TagOrContent, _> = visitor.visit_map(DummyMapAccess);
    assert!(result.is_ok());
    if let Ok(tag_or_content) = result {
        match tag_or_content {
            TagOrContent::Content(Content::Map(map)) => {
                assert!(map.is_empty());
            }
            _ => panic!("Expected Content variant"),
        }
    }
}

#[test]
fn test_visit_map_with_single_entry() {
    struct SingleEntryMapAccess {
        key: Option<String>,
    }

    impl<'de> MapAccess<'de> for SingleEntryMapAccess {
        type Error = &'static str;

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<K::Value>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            if self.key.is_none() {
                self.key = Some("key".to_string());
                Ok(Some("<YourKeySeed>".into()))
            } else {
                Ok(None)
            }
        }

        fn next_value_seed<V>(&mut self, _: V) -> Result<V::Value, Self::Error>
        where
            V: DeserializeSeed<'de>,
        {
            Ok(Content::String("value".to_string()))
        }
    }

    let visitor = TagOrContentVisitor {
        name: "test",
        value: PhantomData,
    };
    let result: Result<TagOrContent, _> = visitor.visit_map(SingleEntryMapAccess { key: None });
    assert!(result.is_ok());
    if let Ok(tag_or_content) = result {
        match tag_or_content {
            TagOrContent::Content(Content::Map(map)) => {
                assert_eq!(map.len(), 1);
                assert_eq!(map[0].0, Content::String("key".to_string()));
                assert_eq!(map[0].1, Content::String("value".to_string()));
            }
            _ => panic!("Expected Content variant"),
        }
    }
}


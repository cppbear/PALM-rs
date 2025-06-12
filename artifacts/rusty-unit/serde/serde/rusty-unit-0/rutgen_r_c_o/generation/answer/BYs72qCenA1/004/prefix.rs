// Answer 0

#[test]
fn test_visit_map_with_duplicate_tag() {
    struct MockMap {
        items: Vec<(TagOrContent<'static>, Content<'static>)>,
        current: usize,
    }

    impl MockMap {
        fn new(items: Vec<(TagOrContent<'static>, Content<'static>)>) -> Self {
            MockMap { items, current: 0 }
        }
    }

    impl MapAccess<'static> for MockMap {
        type Error = serde::de::Error;

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent<'static>>, Self::Error> 
        where K: DeserializeSeed<'static> {
            if self.current < self.items.len() {
                let item = &self.items[self.current];
                self.current += 1;
                Ok(Some(item.0.clone()))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<Content<'static>, Self::Error> 
        where V: Deserialize<'static> {
            Err(de::Error::duplicate_field("tag_name"))
        }
    }

    let mut items = Vec::new();
    items.push((TagOrContent::Tag, Content::String("tag_value".to_string())));
    items.push((TagOrContent::Tag, Content::String("tag_value_duplicate".to_string())));

    let mock_map = MockMap::new(items);
    let visitor = TaggedContentVisitor::<T> {
        tag_name: "tag_name",
        expecting: "Expected content map",
        value: PhantomData,
    };

    let result = visitor.visit_map(mock_map);
}

#[test]
fn test_visit_map_with_missing_tag() {
    struct MockMap {
        items: Vec<(TagOrContent<'static>, Content<'static>)>,
        current: usize,
    }

    impl MockMap {
        fn new(items: Vec<(TagOrContent<'static>, Content<'static>)>) -> Self {
            MockMap { items, current: 0 }
        }
    }

    impl MapAccess<'static> for MockMap {
        type Error = serde::de::Error;

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent<'static>>, Self::Error> 
        where K: DeserializeSeed<'static> {
            if self.current < self.items.len() {
                let item = &self.items[self.current];
                self.current += 1;
                Ok(Some(item.0.clone()))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<Content<'static>, Self::Error> 
        where V: Deserialize<'static> {
            Ok(Content::String("value".to_string()))
        }
    }

    let items = vec![
        (TagOrContent::Content(Content::String("key".to_string())), Content::String("value".to_string()))
    ];

    let mock_map = MockMap::new(items);
    let visitor = TaggedContentVisitor::<T> {
        tag_name: "tag_name",
        expecting: "Expected content map",
        value: PhantomData,
    };

    let result = visitor.visit_map(mock_map);
}

#[test]
fn test_visit_map_with_only_tag() {
    struct MockMap {
        items: Vec<(TagOrContent<'static>, Content<'static>)>,
        current: usize,
    }

    impl MockMap {
        fn new(items: Vec<(TagOrContent<'static>, Content<'static>)>) -> Self {
            MockMap { items, current: 0 }
        }
    }

    impl MapAccess<'static> for MockMap {
        type Error = serde::de::Error;

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent<'static>>, Self::Error> 
        where K: DeserializeSeed<'static> {
            if self.current < self.items.len() {
                let item = &self.items[self.current];
                self.current += 1;
                Ok(Some(item.0.clone()))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<Content<'static>, Self::Error> 
        where V: Deserialize<'static> {
            Ok(Content::String("value".to_string()))
        }
    }

    let items = vec![
        (TagOrContent::Tag, Content::String("tag_value".to_string()))
    ];

    let mock_map = MockMap::new(items);
    let visitor = TaggedContentVisitor::<T> {
        tag_name: "tag_name",
        expecting: "Expected content map",
        value: PhantomData,
    };

    let result = visitor.visit_map(mock_map);
}


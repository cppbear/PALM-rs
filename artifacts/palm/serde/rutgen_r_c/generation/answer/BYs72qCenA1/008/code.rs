// Answer 0

fn test_visit_map_success() {
    struct MockMapAccess {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = &'static str;

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            K: Seed<'de, Value = TagOrContent<'de>>,
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
                self.index += 1;
                Ok(value)
            } else {
                Err("No more values")
            }
        }
    }

    let visitor = TaggedContentVisitor {
        tag_name: "tag_name",
        expecting: "expected",
        value: PhantomData,
    };

    let keys = vec![
        TagOrContent::Tag,
        TagOrContent::Content(Content::String("key".to_string())),
    ];
    
    let values = vec![
        Content::String("tag_value".to_string()),
        Content::String("content_value".to_string()),
    ];

    let mut map_access = MockMapAccess { keys, values, index: 0 };

    let result: Result<(Content<'static>, Content<'static>), _> = visitor.visit_map(&mut map_access);
    assert!(result.is_ok());
}

fn test_visit_map_missing_field() {
    struct MockMapAccess {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = &'static str;

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            K: Seed<'de, Value = TagOrContent<'de>>,
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
                self.index += 1;
                Ok(value)
            } else {
                Err("No more values")
            }
        }
    }

    let visitor = TaggedContentVisitor {
        tag_name: "tag_name",
        expecting: "expected",
        value: PhantomData,
    };

    let keys = vec![TagOrContent::Content(Content::String("key".to_string()))];
    let values = vec![Content::String("content_value".to_string())];

    let mut map_access = MockMapAccess { keys, values, index: 0 };

    let result: Result<(Content<'static>, Content<'static>), _> = visitor.visit_map(&mut map_access);
    assert!(result.is_err());
}

fn test_visit_map_duplicate_field() {
    struct MockMapAccess {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = &'static str;

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            K: Seed<'de, Value = TagOrContent<'de>>,
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
                self.index += 1;
                Ok(value)
            } else {
                Err("No more values")
            }
        }
    }

    let visitor = TaggedContentVisitor {
        tag_name: "tag_name",
        expecting: "expected",
        value: PhantomData,
    };

    let keys = vec![TagOrContent::Tag, TagOrContent::Tag];
    let values = vec![Content::String("tag_value".to_string()), Content::String("tag_value_2".to_string())];

    let mut map_access = MockMapAccess { keys, values, index: 0 };

    let result: Result<(Content<'static>, Content<'static>), _> = visitor.visit_map(&mut map_access);
    assert!(result.is_err());
}


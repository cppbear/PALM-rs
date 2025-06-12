// Answer 0

fn test_visit_map_with_valid_content() {
    struct MockMapAccess {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = ();

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            K: DeserializeSeed<'de>,
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
                Err(())
            }
        }
    }

    let map = MockMapAccess {
        keys: vec![
            TagOrContent::Content(Content::String("key1".to_string())),
            TagOrContent::Tag,
        ],
        values: vec![Content::String("value1".to_string()), Content::String("tag_value".to_string())],
        index: 0,
    };

    let visitor = TaggedContentVisitor::<T> {
        tag_name: "tag",
        expecting: "expected content",
        value: PhantomData,
    };

    let result = visitor.visit_map(map);
    assert!(result.is_ok());
}

fn test_visit_map_with_missing_tag() {
    struct MockMapAccess {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = ();

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            K: DeserializeSeed<'de>,
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
                Err(())
            }
        }
    }

    let map = MockMapAccess {
        keys: vec![TagOrContent::Content(Content::String("key1".to_string()))],
        values: vec![Content::String("value1".to_string())],
        index: 0,
    };

    let visitor = TaggedContentVisitor::<T> {
        tag_name: "tag",
        expecting: "expected content",
        value: PhantomData,
    };

    let result = visitor.visit_map(map);
    assert!(result.is_err());
}

fn test_visit_map_with_duplicate_tag() {
    struct MockMapAccess {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = ();

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            K: DeserializeSeed<'de>,
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
                Err(())
            }
        }
    }

    let map = MockMapAccess {
        keys: vec![
            TagOrContent::Tag,
            TagOrContent::Tag,
        ],
        values: vec![Content::String("tag_value".to_string()), Content::String("tag_value2".to_string())],
        index: 0,
    };

    let visitor = TaggedContentVisitor::<T> {
        tag_name: "tag",
        expecting: "expected content",
        value: PhantomData,
    };

    let result = visitor.visit_map(map);
    assert!(result.is_err());
}


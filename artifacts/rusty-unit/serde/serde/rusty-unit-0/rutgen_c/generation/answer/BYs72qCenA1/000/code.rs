// Answer 0

#[test]
fn test_visit_map_with_missing_field() {
    struct MockMapAccess {
        keys: Vec<Option<TagOrContent<'static>>>,
        values: Vec<Option<Content<'static>>>,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = &'static str;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            Ok(self.keys.pop().unwrap_or(None))
        }

        fn next_value<V>(&mut self) -> Result<Content<'de>, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Ok(self.values.pop().unwrap_or(None).unwrap())
        }
    }

    let mut map_access = MockMapAccess {
        keys: vec![None],
        values: vec![],
    };

    let visitor = TaggedContentVisitor {
        tag_name: "tag",
        expecting: "a map with a tag",
        value: std::marker::PhantomData,
    };

    let result = visitor.visit_map(map_access);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_with_duplicate_field() {
    struct MockMapAccess {
        keys: Vec<TagOrContent<'static>>,
        idx: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = &'static str;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            if self.idx < self.keys.len() {
                let key = self.keys[self.idx].clone();
                self.idx += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<Content<'de>, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Ok(Content::U8(1)) // Return a dummy value
        }
    }

    let mut map_access = MockMapAccess {
        keys: vec![TagOrContent::Tag, TagOrContent::Tag], // Duplicate tag
        idx: 0,
    };

    let visitor = TaggedContentVisitor {
        tag_name: "tag",
        expecting: "a map with a tag",
        value: std::marker::PhantomData,
    };

    let result = visitor.visit_map(map_access);
    assert!(result.is_err());
}

#[test]
fn test_visit_map_with_valid_data() {
    struct MockMapAccess {
        keys: Vec<TagOrContent<'static>>,
        idx: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = &'static str;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            if self.idx < self.keys.len() {
                let key = self.keys[self.idx].clone();
                self.idx += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<Content<'de>, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Ok(Content::U8(1)) // Return a dummy value
        }
    }

    let mut map_access = MockMapAccess {
        keys: vec![TagOrContent::Tag, TagOrContent::Content(Content::U8(42))],
        idx: 0,
    };

    let visitor = TaggedContentVisitor {
        tag_name: "tag",
        expecting: "a map with a tag",
        value: std::marker::PhantomData,
    };

    let result = visitor.visit_map(map_access);
    assert!(result.is_ok());
    let (tag, content) = result.unwrap();
    assert_eq!(tag, Content::U8(1)); // Assuming the dummy value for the tag
}


// Answer 0

#[test]
fn test_visit_map_with_valid_tag_and_content() {
    struct MockMap {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
        index: usize,
    }

    impl MapAccess<'static> for MockMap {
        type Error = serde::de::value::Error;

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent<'static>>, Self::Error>
        where
            K: serde::de::DeserializeSeed<'static>,
        {
            if self.index < self.keys.len() {
                let key = Some(self.keys[self.index].clone());
                self.index += 1;
                Ok(key)
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: serde::de::Deserialize<'static>,
        {
            if self.index <= self.values.len() {
                Ok(self.values[self.index - 1].clone().into())
            } else {
                Err(serde::de::value::Error::custom("No more values"))
            }
        }
    }

    let map = MockMap {
        keys: vec![
            TagOrContent::Tag,
            TagOrContent::Content(Content::String("key1".to_string())),
            TagOrContent::Content(Content::String("key2".to_string())),
        ],
        values: vec![
            Content::String("tag_value".to_string()),
            Content::String("value1".to_string()),
            Content::String("value2".to_string()),
        ],
        index: 0,
    };

    let visitor = TaggedContentVisitor {
        tag_name: "tag_name",
        expecting: "expecting_value",
        value: PhantomData,
    };

    let result = visitor.visit_map(map);
}

#[test]
fn test_visit_map_with_duplicate_tag() {
    struct MockMap {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
        index: usize,
    }

    impl MapAccess<'static> for MockMap {
        type Error = serde::de::value::Error;

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent<'static>>, Self::Error>
        where
            K: serde::de::DeserializeSeed<'static>,
        {
            if self.index < self.keys.len() {
                let key = Some(self.keys[self.index].clone());
                self.index += 1;
                Ok(key)
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: serde::de::Deserialize<'static>,
        {
            if self.index <= self.values.len() {
                Ok(self.values[self.index - 1].clone().into())
            } else {
                Err(serde::de::value::Error::custom("No more values"))
            }
        }
    }

    let map = MockMap {
        keys: vec![
            TagOrContent::Tag,
            TagOrContent::Tag,
            TagOrContent::Content(Content::String("key1".to_string())),
        ],
        values: vec![
            Content::String("tag1".to_string()),
            Content::String("tag2".to_string()),
            Content::String("value1".to_string()),
        ],
        index: 0,
    };

    let visitor = TaggedContentVisitor {
        tag_name: "tag_name",
        expecting: "expecting_value",
        value: PhantomData,
    };

    let result = visitor.visit_map(map);
}

#[test]
fn test_visit_map_with_missing_tag() {
    struct MockMap {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
        index: usize,
    }

    impl MapAccess<'static> for MockMap {
        type Error = serde::de::value::Error;

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent<'static>>, Self::Error>
        where
            K: serde::de::DeserializeSeed<'static>,
        {
            if self.index < self.keys.len() {
                let key = Some(self.keys[self.index].clone());
                self.index += 1;
                Ok(key)
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: serde::de::Deserialize<'static>,
        {
            if self.index <= self.values.len() {
                Ok(self.values[self.index - 1].clone().into())
            } else {
                Err(serde::de::value::Error::custom("No more values"))
            }
        }
    }

    let map = MockMap {
        keys: vec![
            TagOrContent::Content(Content::String("key1".to_string())),
            TagOrContent::Content(Content::String("key2".to_string())),
        ],
        values: vec![
            Content::String("value1".to_string()),
            Content::String("value2".to_string()),
        ],
        index: 0,
    };

    let visitor = TaggedContentVisitor {
        tag_name: "tag_name",
        expecting: "expecting_value",
        value: PhantomData,
    };

    let result = visitor.visit_map(map);
}


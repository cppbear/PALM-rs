// Answer 0

fn test_visit_map_duplicate_tag() {
    struct MockMapAccess {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = Box<dyn std::error::Error>;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<TagOrContent<'de>>, Self::Error>
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

        fn next_value<V>(&mut self) -> Result<Content<'de>, Self::Error>
        where
            V: Deserialize<'de>,
        {
            if self.index - 1 < self.values.len() {
                Ok(self.values[self.index - 1].clone())
            } else {
                Err("No more values".into())
            }
        }
    }

    let visitor = TaggedContentVisitor::<T> {
        tag_name: "tag_name",
        expecting: "expected",
        value: std::marker::PhantomData,
    };

    let keys = vec![
        TagOrContent::Tag,
        TagOrContent::Tag, // Duplicate Tag
        TagOrContent::Content(Content::Bool(true)),
    ];
    let values = vec![
        Content::U8(1),
        Content::U8(2), // Value for the duplicate tag
        Content::U32(32),
    ];

    let mut map_access = MockMapAccess {
        keys,
        values,
        index: 0,
    };

    let result = visitor.visit_map(&mut map_access);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "duplicate field `tag_name`");
}


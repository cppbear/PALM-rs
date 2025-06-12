// Answer 0

#[test]
fn test_visit_map_duplicate_field_error() {
    struct MockMapAccess {
        entries: Vec<(Content, Content)>,
        index: usize,
    }

    impl MapAccess<'static> for MockMapAccess {
        type Error = ();
        
        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<TagOrContent<'static>>, Self::Error>
        where
            K: DeserializeSeed<'static>,
        {
            if self.index < self.entries.len() {
                let key = self.entries[self.index].0.clone();
                self.index += 1;
                Ok(Some(TagOrContent::Tag)) // Trigger duplicate field
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<Content<'static>, Self::Error>
        where
            V: Deserialize<'static>,
        {
            if self.index <= self.entries.len() {
                let value = self.entries[self.index - 1].1.clone();
                Ok(value)
            } else {
                Err(())
            }
        }
    }

    let entries = vec![
        (Content::String("tag".into()), Content::U32(1)), // First tag
        (Content::String("field1".into()), Content::U32(2)),
        (Content::String("field2".into()), Content::U32(3)),
        (Content::String("tag".into()), Content::U32(4)), // Duplicate tag
    ];

    let mock_map = MockMapAccess {
        entries,
        index: 0,
    };

    let visitor = TaggedContentVisitor::<u32> {
        tag_name: "tag",
        expecting: "Expected a map with a tag",
        value: PhantomData,
    };

    let _ = visitor.visit_map(mock_map);
}


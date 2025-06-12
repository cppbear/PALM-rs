// Answer 0

fn test_visit_map_duplicate_tag() {
    struct MockMapAccess {
        calls: usize,
        key: Option<TagOrContent<'static>>,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = &'static str;

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent<'de>>, Self::Error> {
            if self.calls == 0 {
                self.calls += 1;
                Ok(Some(TagOrContent::Tag))
            } else {
                Err("duplicate field")
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error> {
            Err("value error")
        }
    }

    let visitor = TaggedContentVisitor {
        tag_name: "tag",
        expecting: "Expected",
        value: PhantomData,
    };
    let mut mock_map = MockMapAccess { calls: 0, key: None };
    let result: Result<(Content<'static>, Content<'static>), _> = visitor.visit_map(&mut mock_map);
    assert!(result.is_err());
}

fn test_visit_map_missing_tag() {
    struct MockMapAccess {
        calls: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = &'static str;

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent<'de>>, Self::Error> {
            Ok(None)
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error> {
            Ok(unsafe { std::mem::zeroed() }) // assuming it won't be called
        }
    }

    let visitor = TaggedContentVisitor {
        tag_name: "tag",
        expecting: "Expected",
        value: PhantomData,
    };
    let mut mock_map = MockMapAccess { calls: 0 };
    let result: Result<(Content<'static>, Content<'static>), _> = visitor.visit_map(&mut mock_map);
    assert!(result.is_err());
}

fn test_visit_map_successful() {
    struct MockMapAccess {
        calls: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = &'static str;

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent<'de>>, Self::Error> {
            if self.calls == 0 {
                self.calls += 1;
                Ok(Some(TagOrContent::Tag))
            } else {
                Ok(Some(TagOrContent::Content(Content::String("key".to_string()))))
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error> {
            Ok(unsafe { std::mem::zeroed() }) // Mock value
        }
    }

    let visitor = TaggedContentVisitor {
        tag_name: "tag",
        expecting: "Expected",
        value: PhantomData,
    };
    let mut mock_map = MockMapAccess { calls: 0 };
    let result: Result<(Content<'static>, Content<'static>), _> = visitor.visit_map(&mut mock_map);
    assert!(result.is_ok());
}


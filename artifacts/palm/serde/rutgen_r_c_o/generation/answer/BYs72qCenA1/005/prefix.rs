// Answer 0

#[test]
fn test_visit_map_with_valid_tag_and_content() {
    struct MockMap {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
        index: usize,
    }

    impl MapAccess<'static> for MockMap {
        type Error = ();

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<Self::Key>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index].clone();
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<Self::Value, Self::Error> {
            if self.index - 1 < self.values.len() {
                Ok(self.values[self.index - 1].clone())
            } else {
                Err(())
            }
        }
    }

    let map = MockMap {
        keys: vec![TagOrContent::Tag, TagOrContent::Content(Content::Str("test"))],
        values: vec![Content::String("value".to_string()), Content::Bool(true)],
        index: 0,
    };

    let visitor = TaggedContentVisitor::<T> {
        tag_name: "test_tag",
        expecting: "expecting a test",
        value: PhantomData,
    };

    let _ = visitor.visit_map(map);
}

#[test]
fn test_visit_map_with_missing_tag() {
    struct MockMap {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
        index: usize,
    }

    impl MapAccess<'static> for MockMap {
        type Error = ();

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<Self::Key>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index].clone();
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<Self::Value, Self::Error> {
            if self.index - 1 < self.values.len() {
                Ok(self.values[self.index - 1].clone())
            } else {
                Err(())
            }
        }
    }

    let map = MockMap {
        keys: vec![TagOrContent::Content(Content::Str("test"))],
        values: vec![Content::String("value".to_string())],
        index: 0,
    };

    let visitor = TaggedContentVisitor::<T> {
        tag_name: "test_tag",
        expecting: "expecting a test",
        value: PhantomData,
    };

    let result = visitor.visit_map(map);
    let _ = result.unwrap_err();
}

#[test]
fn test_visit_map_with_duplicate_tag() {
    struct MockMap {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
        index: usize,
    }

    impl MapAccess<'static> for MockMap {
        type Error = ();

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<Self::Key>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index].clone();
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<Self::Value, Self::Error> {
            if self.index - 1 < self.values.len() {
                Ok(self.values[self.index - 1].clone())
            } else {
                Err(())
            }
        }
    }

    let map = MockMap {
        keys: vec![
            TagOrContent::Tag,
            TagOrContent::Tag,
            TagOrContent::Content(Content::Str("test")),
        ],
        values: vec![Content::String("value".to_string()), Content::Bool(true)],
        index: 0,
    };

    let visitor = TaggedContentVisitor::<T> {
        tag_name: "test_tag",
        expecting: "expecting a test",
        value: PhantomData,
    };

    let result = visitor.visit_map(map);
    let _ = result.unwrap_err();
}

#[test]
fn test_visit_map_with_invalid_key() {
    struct MockMap {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
        index: usize,
    }

    impl MapAccess<'static> for MockMap {
        type Error = ();

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<Self::Key>, Self::Error> {
            if self.index < self.keys.len() {
                let key = self.keys[self.index].clone();
                self.index += 1;
                Ok(Some(key))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<Self::Value, Self::Error> {
            if self.index - 1 < self.values.len() {
                Ok(self.values[self.index - 1].clone())
            } else {
                Err(())
            }
        }
    }

    let map = MockMap {
        keys: vec![TagOrContent::Content(Content::Str("invalid"))],
        values: vec![Content::String("value".to_string())],
        index: 0,
    };

    let visitor = TaggedContentVisitor::<T> {
        tag_name: "test_tag",
        expecting: "expecting a test",
        value: PhantomData,
    };

    let result = visitor.visit_map(map);
    let _ = result.unwrap_err();
}


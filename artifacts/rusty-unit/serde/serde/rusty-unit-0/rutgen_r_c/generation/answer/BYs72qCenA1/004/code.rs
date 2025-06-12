// Answer 0

fn test_visit_map_missing_field() {
    struct MockMapAccess {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = ();
        fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error>
        where
            K: Deserialize<'de>,
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
            if self.index - 1 < self.values.len() {
                let value = self.values[self.index - 1].clone();
                self.index += 1;
                Ok(value)
            } else {
                Err(())
            }
        }
    }

    let map_access = MockMapAccess {
        keys: vec![TagOrContent::Tag, TagOrContent::Content(Content::Bool(true))],
        values: vec![Content::None],
        index: 0,
    };

    let visitor = TaggedContentVisitor::<T> {
        tag_name: "tag",
        expecting: "",
        value: PhantomData,
    };

    let result: Result<(T, Content), ()> = visitor.visit_map(map_access);
    assert!(result.is_err());
}

fn test_visit_map_duplicate_field() {
    struct MockMapAccess {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = ();
        fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error>
        where
            K: Deserialize<'de>,
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
            if self.index - 1 < self.values.len() {
                let value = self.values[self.index - 1].clone();
                self.index += 1;
                Ok(value)
            } else {
                Err(())
            }
        }
    }

    let map_access = MockMapAccess {
        keys: vec![TagOrContent::Tag, TagOrContent::Tag, TagOrContent::Content(Content::Bool(true))],
        values: vec![Content::None, Content::None],
        index: 0,
    };

    let visitor = TaggedContentVisitor::<T> {
        tag_name: "tag",
        expecting: "",
        value: PhantomData,
    };

    let result: Result<(T, Content), ()> = visitor.visit_map(map_access);
    assert!(result.is_err());
}

fn test_visit_map_success() {
    struct MockMapAccess {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = ();
        fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error>
        where
            K: Deserialize<'de>,
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
            if self.index - 1 < self.values.len() {
                let value = self.values[self.index - 1].clone();
                self.index += 1;
                Ok(value)
            } else {
                Err(())
            }
        }
    }

    let map_access = MockMapAccess {
        keys: vec![TagOrContent::Tag, TagOrContent::Content(Content::Bool(true))],
        values: vec![Content::None],
        index: 0,
    };

    let visitor = TaggedContentVisitor::<T> {
        tag_name: "tag",
        expecting: "",
        value: PhantomData,
    };

    let result: Result<(T, Content), ()> = visitor.visit_map(map_access);
    assert!(result.is_ok());
}


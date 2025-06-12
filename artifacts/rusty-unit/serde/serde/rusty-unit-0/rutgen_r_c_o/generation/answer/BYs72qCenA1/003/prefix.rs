// Answer 0

#[test]
fn test_visit_map_no_tag() {
    struct MockMapAccess {
        calls: usize,
        size_hint: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = ();
        
        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            self.calls += 1;
            if self.calls > self.size_hint {
                Ok(None)
            } else {
                Ok(Some(TagOrContent::Content(Content::Bool(true))))
            }
        }

        fn next_value<V>(&mut self) -> Result<Content<'de>, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Err(())
        }

        fn size_hint(&self) -> Option<usize> {
            Some(self.size_hint)
        }
    }

    let mock = MockMapAccess { calls: 0, size_hint: 1 };
    let visitor = TaggedContentVisitor::<T> {
        tag_name: "tag",
        expecting: "A valid tag",
        value: PhantomData,
    };

    let _ = visitor.visit_map(mock);
}

#[test]
fn test_visit_map_duplicate_tag() {
    struct MockMapAccess {
        calls: usize,
        size_hint: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = ();

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            self.calls += 1;
            if self.calls == 1 {
                Ok(Some(TagOrContent::Tag))
            } else {
                Ok(Some(TagOrContent::Tag))
            }
        }

        fn next_value<V>(&mut self) -> Result<Content<'de>, Self::Error>
        where
            V: Deserialize<'de>,
        {
            Ok(Content::Bool(true))
        }

        fn size_hint(&self) -> Option<usize> {
            Some(self.size_hint)
        }
    }

    let mock = MockMapAccess { calls: 0, size_hint: 1 };
    let visitor = TaggedContentVisitor::<T> {
        tag_name: "tag",
        expecting: "A valid tag",
        value: PhantomData,
    };

    let _ = visitor.visit_map(mock);
}


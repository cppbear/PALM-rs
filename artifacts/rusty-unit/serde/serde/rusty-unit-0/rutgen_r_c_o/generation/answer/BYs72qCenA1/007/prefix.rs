// Answer 0

#[test]
fn test_visit_map_with_duplicate_tag() {
    struct TestMapAccess {
        called: bool,
        key: Option<TagOrContent<'static>>,
        value: Option<Content>,
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = serde::de::Error;

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            if self.called {
                Ok(Some(TagOrContent::Tag))
            } else {
                self.called = true;
                Ok(Some(TagOrContent::Tag))
            }
        }

        fn next_value<D>(&mut self) -> Result<Content<'de>, Self::Error>
        where
            D: Deserializer<'de>,
        {
            Err(serde::de::Error::custom("Error"))
        }
    }

    let map_access = TestMapAccess { called: false, key: None, value: None };
    let visitor = TaggedContentVisitor::<String> { tag_name: "test_tag", expecting: "test", value: PhantomData };
    let _ = visitor.visit_map(map_access);
}

#[test]
fn test_visit_map_with_missing_tag() {
    struct TestMapAccess {
        called: bool,
    }

    impl<'de> MapAccess<'de> for TestMapAccess {
        type Error = serde::de::Error;

        fn next_key_seed<K>(&mut self, _: K) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            K: DeserializeSeed<'de>,
        {
            Ok(None)
        }

        fn next_value<D>(&mut self) -> Result<Content<'de>, Self::Error>
        where
            D: Deserializer<'de>,
        {
            Err(serde::de::Error::custom("Error"))
        }
    }

    let map_access = TestMapAccess { called: false };
    let visitor = TaggedContentVisitor::<String> { tag_name: "test_tag", expecting: "test", value: PhantomData };
    let _ = visitor.visit_map(map_access);
}


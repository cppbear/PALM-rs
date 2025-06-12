// Answer 0

fn test_visit_map_missing_tag() {
    struct MockMapAccess {
        calls: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = std::convert::Infallible;

        fn next_key_seed<K>(&mut self, _key_seed: K) -> Result<Option<TagOrContent<'de>>, Self::Error>
        where
            K: serde::de::DeserializeSeed<'de>,
        {
            if self.calls == 0 {
                self.calls += 1;
                Ok(Some(TagOrContent::Content(Content::Str("content"))))
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error>
        where
            V: serde::de::Deserialize<'de>,
        {
            if self.calls == 1 {
                // Return a value only after the key has been read
                Ok(Content::Str("value").into())
            } else {
                Err(std::convert::Infallible)
            }
        }
    }

    let visitor = TaggedContentVisitor::<()>::new("tag");
    let map_access = MockMapAccess { calls: 0 };
    let result = visitor.visit_map(map_access);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), de::Error::missing_field("tag"));
}


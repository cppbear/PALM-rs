// Answer 0

#[test]
fn test_visit_map_missing_field() {
    struct MockMapAccess {
        keys: Vec<TagOrContent>,
        values: Vec<Content>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MockMapAccess {
        type Error = de::Error;

        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<Self::Key>, Self::Error> 
        where 
            K: DeserializeSeed<'de> {
            if self.index < self.keys.len() {
                let result = Some(self.keys[self.index]);
                self.index += 1;
                Ok(result)
            } else {
                Ok(None)
            }
        }

        fn next_value<V>(&mut self) -> Result<V, Self::Error> 
        where 
            V: Deserialize<'de> {
            if self.index <= self.values.len() {
                Ok(self.values[self.index - 1].clone().into_deserialized())
            } else {
                Err(de::Error::custom("No more values"))
            }
        }

        fn size_hint(&self) -> Option<usize> {
            Some(self.keys.len())
        }
    }

    let mut map = MockMapAccess {
        keys: vec![],
        values: vec![],
        index: 0,
    };

    #[derive(Deserialize)]
    struct Context {
        tag_name: &'static str,
    }

    let context = Context { tag_name: "tag" };

    let result: Result<(Content, Content), de::Error> = visit_map(map);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), de::Error::missing_field("tag"));
}


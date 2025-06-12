// Answer 0

#[test]
fn test_visit_map_empty() {
    struct EmptyMapAccess;
    
    impl<'de> MapAccess<'de> for EmptyMapAccess {
        type Error = ();

        fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error> where K: Deserialize<'de> {
            Ok(None)
        }

        fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error> where K: Deserialize<'de>, V: Deserialize<'de> {
            Ok(None)
        }

        fn size_hint(&self) -> (Option<usize>, Option<usize>) {
            (Some(0), Some(0))
        }
    }

    let visitor = ContentVisitor { value: PhantomData };
    let result: Result<Content, ()> = visitor.visit_map(EmptyMapAccess);
    assert_eq!(result.unwrap(), Content::Map(vec![]));
}

#[test]
fn test_visit_map_single_entry() {
    struct SingleEntryMapAccess {
        count: usize,
    }
    
    impl<'de> MapAccess<'de> for SingleEntryMapAccess {
        type Error = ();

        fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error> where K: Deserialize<'de> {
            if self.count == 0 {
                self.count += 1;
                Ok(Some(Content::String("key".to_string()) as K))
            } else {
                Ok(None)
            }
        }

        fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error> where K: Deserialize<'de>, V: Deserialize<'de> {
            if self.count == 1 {
                self.count += 1;
                Ok(Some((Content::String("key".to_string()) as K, Content::String("value".to_string()) as V)))
            } else {
                Ok(None)
            }
        }

        fn size_hint(&self) -> (Option<usize>, Option<usize>) {
            (Some(1), Some(1))
        }
    }

    let visitor = ContentVisitor { value: PhantomData };
    let result: Result<Content, ()> = visitor.visit_map(SingleEntryMapAccess { count: 0 });
    assert_eq!(result.unwrap(), Content::Map(vec![(Content::String("key".to_string()), Content::String("value".to_string()))]));
}

#[test]
fn test_visit_map_multiple_entries() {
    struct MultiEntryMapAccess {
        count: usize,
    }
    
    impl<'de> MapAccess<'de> for MultiEntryMapAccess {
        type Error = ();

        fn next_key<K>(&mut self) -> Result<Option<K>, Self::Error> where K: Deserialize<'de> {
            if self.count < 2 {
                self.count += 1;
                Ok(Some(Content::String(format!("key{}", self.count - 1)) as K))
            } else {
                Ok(None)
            }
        }

        fn next_entry<K, V>(&mut self) -> Result<Option<(K, V)>, Self::Error> where K: Deserialize<'de>, V: Deserialize<'de> {
            if self.count <= 2 {
                let key = Content::String(format!("key{}", self.count - 1)) as K;
                let value = Content::String(format!("value{}", self.count - 1)) as V;
                Ok(Some((key, value)))
            } else {
                Ok(None)
            }
        }

        fn size_hint(&self) -> (Option<usize>, Option<usize>) {
            (Some(2), Some(2))
        }
    }

    let visitor = ContentVisitor { value: PhantomData };
    let result: Result<Content, ()> = visitor.visit_map(MultiEntryMapAccess { count: 0 });
    assert_eq!(result.unwrap(), Content::Map(vec![
        (Content::String("key0".to_string()), Content::String("value0".to_string())),
        (Content::String("key1".to_string()), Content::String("value1".to_string()))
    ]));
}


// Answer 0

#[test]
fn test_visit_map_with_empty_map() {
    struct MockMap {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
    }
    
    impl<'de> MapAccess<'de> for MockMap {
        type Error = ();
        
        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<TagOrContent<'de>>, Self::Error> 
        where
            K: DeserializeSeed<'de>,
        {
            Ok(self.keys.pop())
        }
        
        fn next_value<V>(&mut self) -> Result<V, Self::Error> 
        where
            V: Deserialize<'de>,
        {
            Ok(self.values.pop().unwrap())
        }
        
        fn size_hint(&self) -> (Option<usize>, Option<usize>) {
            (Some(self.keys.len()), Some(self.keys.len()))
        }
    }

    let visitor = TaggedContentVisitor {
        tag_name: "test_tag",
        expecting: "expected something",
        value: PhantomData,
    };

    let mut map = MockMap {
        keys: Vec::new(),
        values: Vec::new(),
    };

    let _ = visitor.visit_map(map);
}

#[test]
fn test_visit_map_with_duplicate_tag() {
    struct MockMap {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
    }
    
    impl<'de> MapAccess<'de> for MockMap {
        type Error = ();
        
        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<TagOrContent<'de>>, Self::Error> 
        where
            K: DeserializeSeed<'de>,
        {
            Ok(self.keys.pop())
        }
        
        fn next_value<V>(&mut self) -> Result<V, Self::Error> 
        where
            V: Deserialize<'de>,
        {
            Ok(self.values.pop().unwrap())
        }
        
        fn size_hint(&self) -> (Option<usize>, Option<usize>) {
            (Some(self.keys.len()), Some(self.keys.len()))
        }
    }

    let visitor = TaggedContentVisitor {
        tag_name: "test_tag",
        expecting: "expected something",
        value: PhantomData,
    };

    let mut map = MockMap {
        keys: vec![TagOrContent::Tag, TagOrContent::Tag],
        values: vec![Content::Bool(true)],
    };

    let _ = visitor.visit_map(map);
}

#[test]
fn test_visit_map_with_missing_tag() {
    struct MockMap {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
    }
    
    impl<'de> MapAccess<'de> for MockMap {
        type Error = ();
        
        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<TagOrContent<'de>>, Self::Error> 
        where
            K: DeserializeSeed<'de>,
        {
            Ok(self.keys.pop())
        }
        
        fn next_value<V>(&mut self) -> Result<V, Self::Error> 
        where
            V: Deserialize<'de>,
        {
            Ok(self.values.pop().unwrap())
        }
        
        fn size_hint(&self) -> (Option<usize>, Option<usize>) {
            (Some(self.keys.len()), Some(self.keys.len()))
        }
    }

    let visitor = TaggedContentVisitor {
        tag_name: "test_tag",
        expecting: "expected something",
        value: PhantomData,
    };

    let mut map = MockMap {
        keys: vec![TagOrContent::Content(Content::Bool(true))],
        values: vec![Content::String("test".to_string())],
    };

    let _ = visitor.visit_map(map);
}

#[test]
fn test_visit_map_with_valid_key_value_pairs() {
    struct MockMap {
        keys: Vec<TagOrContent<'static>>,
        values: Vec<Content<'static>>,
    }
    
    impl<'de> MapAccess<'de> for MockMap {
        type Error = ();
        
        fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<TagOrContent<'de>>, Self::Error> 
        where
            K: DeserializeSeed<'de>,
        {
            Ok(self.keys.pop())
        }
        
        fn next_value<V>(&mut self) -> Result<V, Self::Error> 
        where
            V: Deserialize<'de>,
        {
            Ok(self.values.pop().unwrap())
        }
        
        fn size_hint(&self) -> (Option<usize>, Option<usize>) {
            (Some(self.keys.len()), Some(self.keys.len()))
        }
    }

    let visitor = TaggedContentVisitor {
        tag_name: "test_tag",
        expecting: "expected something",
        value: PhantomData,
    };

    let mut map = MockMap {
        keys: vec![TagOrContent::Tag],
        values: vec![Content::Bool(true)],
    };

    let _ = visitor.visit_map(map);
}


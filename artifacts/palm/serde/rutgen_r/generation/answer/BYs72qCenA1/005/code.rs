// Answer 0

#[test]
fn test_visit_map_success_with_content() {
    struct MapMock {
        keys: Vec<TagOrContent>,
        values: Vec<Content>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MapMock {
        type Error = serde::de::Error;

        fn next_key_seed<T>(&mut self, _: T) -> Result<Option<TagOrContent>, Self::Error> 
        where T: serde::de::DeserializeSeed<'de> {
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
            V: serde::de::Deserialize<'de>,
        {
            if self.index <= self.values.len() {
                let value = self.values[self.index - 1].clone();
                Ok(value as V)
            } else {
                Err(serde::de::Error::custom("No more values available"))
            }
        }
    }

    let mock_map = MapMock {
        keys: vec![TagOrContent::Tag, TagOrContent::Content(Content::new("key1")), TagOrContent::Content(Content::new("key2"))],
        values: vec![Content::new("value1"), Content::new("value2")],
        index: 0,
    };

    let visitor = TagOrContentVisitor::new("tag_name");
    let result = visitor.visit_map(mock_map);
    
    assert!(result.is_ok());
}

#[test]
fn test_visit_map_missing_tag() {
    struct MapMock {
        keys: Vec<TagOrContent>,
        values: Vec<Content>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MapMock {
        type Error = serde::de::Error;

        fn next_key_seed<T>(&mut self, _: T) -> Result<Option<TagOrContent>, Self::Error> 
        where T: serde::de::DeserializeSeed<'de> {
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
            V: serde::de::Deserialize<'de>,
        {
            if self.index <= self.values.len() {
                let value = self.values[self.index - 1].clone();
                Ok(value as V)
            } else {
                Err(serde::de::Error::custom("No more values available"))
            }
        }
    }

    let mock_map = MapMock {
        keys: vec![TagOrContent::Content(Content::new("key1")), TagOrContent::Content(Content::new("key2"))],
        values: vec![Content::new("value1"), Content::new("value2")],
        index: 0,
    };

    let visitor = TagOrContentVisitor::new("tag_name");
    let result = visitor.visit_map(mock_map);
    
    assert!(result.is_err());
}

#[test]
fn test_visit_map_duplicate_tag() {
    struct MapMock {
        keys: Vec<TagOrContent>,
        values: Vec<Content>,
        index: usize,
    }

    impl<'de> MapAccess<'de> for MapMock {
        type Error = serde::de::Error;

        fn next_key_seed<T>(&mut self, _: T) -> Result<Option<TagOrContent>, Self::Error> 
        where T: serde::de::DeserializeSeed<'de> {
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
            V: serde::de::Deserialize<'de>,
        {
            if self.index <= self.values.len() {
                let value = self.values[self.index - 1].clone();
                Ok(value as V)
            } else {
                Err(serde::de::Error::custom("No more values available"))
            }
        }
    }

    let mock_map = MapMock {
        keys: vec![TagOrContent::Tag, TagOrContent::Tag],
        values: vec![Content::new("value1"), Content::new("value2")],
        index: 0,
    };

    let visitor = TagOrContentVisitor::new("tag_name");
    let result = visitor.visit_map(mock_map);
    
    assert!(result.is_err());
}


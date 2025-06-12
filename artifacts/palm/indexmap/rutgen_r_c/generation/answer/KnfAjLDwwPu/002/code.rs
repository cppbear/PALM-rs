// Answer 0

#[test]
fn test_get_key_value_found() {
    struct TestMap {
        data: Vec<(String, i32)>,
    }
    
    impl TestMap {
        fn new() -> Self {
            Self {
                data: vec![("key1".to_string(), 1), ("key2".to_string(), 2)],
            }
        }
        
        fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
        where
            Q: ?Sized + Hash + Equivalent<String>,
        {
            self.data.iter().position(|(k, _)| k == key)
        }
        
        fn as_entries(&self) -> &Vec<(String, i32)> {
            &self.data
        }
        
        fn get_key_value<Q>(&self, key: &Q) -> Option<(&String, &i32)>
        where
            Q: ?Sized + Hash + Equivalent<String>,
        {
            if let Some(i) = self.get_index_of(key) {
                let entry = &self.as_entries()[i];
                Some((&entry.0, &entry.1))
            } else {
                None
            }
        }
    }

    let map = TestMap::new();
    let result = map.get_key_value(&"key1".to_string());
    assert_eq!(result, Some((&"key1".to_string(), &1)));
}

#[test]
fn test_get_key_value_not_found() {
    struct TestMap {
        data: Vec<(String, i32)>,
    }
    
    impl TestMap {
        fn new() -> Self {
            Self {
                data: vec![("key1".to_string(), 1), ("key2".to_string(), 2)],
            }
        }
        
        fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
        where
            Q: ?Sized + Hash + Equivalent<String>,
        {
            self.data.iter().position(|(k, _)| k == key)
        }
        
        fn as_entries(&self) -> &Vec<(String, i32)> {
            &self.data
        }
        
        fn get_key_value<Q>(&self, key: &Q) -> Option<(&String, &i32)>
        where
            Q: ?Sized + Hash + Equivalent<String>,
        {
            if let Some(i) = self.get_index_of(key) {
                let entry = &self.as_entries()[i];
                Some((&entry.0, &entry.1))
            } else {
                None
            }
        }
    }

    let map = TestMap::new();
    let result = map.get_key_value(&"key3".to_string());
    assert_eq!(result, None);
}


// Answer 0

#[test]
fn test_get_mut_not_found() {
    struct TestMap {
        data: Vec<(String, i32)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
        where
            Q: ?Sized + Hash + Equivalent<String>,
        {
            for (i, (k, _)) in self.data.iter().enumerate() {
                if k == key {
                    return Some(i);
                }
            }
            None
        }

        fn as_entries_mut(&mut self) -> &mut Vec<(String, i32)> {
            &mut self.data
        }

        fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut i32>
        where
            Q: ?Sized + Hash + Equivalent<String>,
        {
            if let Some(i) = self.get_index_of(key) {
                let entry = &mut self.as_entries_mut()[i];
                Some(&mut entry.1)
            } else {
                None
            }
        }
    }

    let mut map = TestMap::new();
    map.data.push((String::from("key1"), 100));
    
    // Attempt to get a mutable reference for a key that does not exist
    let result = map.get_mut("non_existent_key");
    assert!(result.is_none());
}

#[test]
fn test_get_mut_empty_map() {
    struct TestMap {
        data: Vec<(String, i32)>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { data: Vec::new() }
        }

        fn get_index_of<Q>(&self, key: &Q) -> Option<usize>
        where
            Q: ?Sized + Hash + Equivalent<String>,
        {
            for (i, (k, _)) in self.data.iter().enumerate() {
                if k == key {
                    return Some(i);
                }
            }
            None
        }

        fn as_entries_mut(&mut self) -> &mut Vec<(String, i32)> {
            &mut self.data
        }

        fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut i32>
        where
            Q: ?Sized + Hash + Equivalent<String>,
        {
            if let Some(i) = self.get_index_of(key) {
                let entry = &mut self.as_entries_mut()[i];
                Some(&mut entry.1)
            } else {
                None
            }
        }
    }

    let mut map = TestMap::new();
    
    // The map is empty, any key lookup should return None
    let result = map.get_mut("non_existent_key");
    assert!(result.is_none());
}


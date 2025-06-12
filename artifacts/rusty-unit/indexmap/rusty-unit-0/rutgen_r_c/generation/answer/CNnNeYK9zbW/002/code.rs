// Answer 0

fn test_get_full_mut_valid_key() {
    struct TestMap {
        entries: Vec<Bucket<i32, String>>,
    }
    
    impl TestMap {
        fn get_index_of(&self, key: &i32) -> Option<usize> {
            self.entries.iter().position(|entry| &entry.key == key)
        }

        fn as_entries_mut(&mut self) -> &mut [Bucket<i32, String>] {
            &mut self.entries
        }

        fn get_full_mut(&mut self, key: &i32) -> Option<(usize, &i32, &mut String)> {
            if let Some(i) = self.get_index_of(key) {
                let entry = &mut self.as_entries_mut()[i];
                Some((i, &entry.key, &mut entry.value))
            } else {
                None
            }
        }
    }
    
    let mut test_map = TestMap {
        entries: vec![
            Bucket { hash: HashValue::default(), key: 1, value: "one".to_string() },
            Bucket { hash: HashValue::default(), key: 2, value: "two".to_string() },
            Bucket { hash: HashValue::default(), key: 3, value: "three".to_string() },
        ],
    };
    
    let result = test_map.get_full_mut(&2);
    
    assert!(result.is_some());
    if let Some((index, key, value)) = result {
        assert_eq!(index, 1);
        assert_eq!(*key, 2);
        assert_eq!(value, &mut "two".to_string());
    }
}

fn test_get_full_mut_key_not_found() {
    struct TestMap {
        entries: Vec<Bucket<i32, String>>,
    }
    
    impl TestMap {
        fn get_index_of(&self, key: &i32) -> Option<usize> {
            self.entries.iter().position(|entry| &entry.key == key)
        }

        fn as_entries_mut(&mut self) -> &mut [Bucket<i32, String>] {
            &mut self.entries
        }

        fn get_full_mut(&mut self, key: &i32) -> Option<(usize, &i32, &mut String)> {
            if let Some(i) = self.get_index_of(key) {
                let entry = &mut self.as_entries_mut()[i];
                Some((i, &entry.key, &mut entry.value))
            } else {
                None
            }
        }
    }
    
    let mut test_map = TestMap {
        entries: vec![
            Bucket { hash: HashValue::default(), key: 1, value: "one".to_string() },
            Bucket { hash: HashValue::default(), key: 2, value: "two".to_string() },
            Bucket { hash: HashValue::default(), key: 3, value: "three".to_string() },
        ],
    };
    
    let result = test_map.get_full_mut(&4);
    
    assert!(result.is_none());
}


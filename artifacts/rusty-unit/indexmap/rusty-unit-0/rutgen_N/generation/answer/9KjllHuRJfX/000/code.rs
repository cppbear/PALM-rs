// Answer 0

#[test]
fn test_swap_remove_full_existing_entry() {
    struct TestMap {
        entries: Vec<(u32, String)>,
        indices: Vec<u32>,
    }
    
    impl TestMap {
        fn new() -> Self {
            Self { entries: Vec::new(), indices: Vec::new() }
        }

        fn insert(&mut self, hash: u32, key: String) {
            self.entries.push((hash, key));
            self.indices.push(hash);
        }

        fn swap_remove_finish(&mut self, index: usize) -> (String, String) {
            let entry = self.entries.swap_remove(index);
            (entry.1, entry.1)
        }

        fn find_entry(&self, hash: u32, eq: impl Fn(&String) -> bool) -> Result<(usize, u32), ()> {
            for (i, (h, k)) in self.entries.iter().enumerate() {
                if *h == hash && eq(k) {
                    return Ok((i, *h));
                }
            }
            Err(())
        }
    }

    impl Equivalent<String> for str {
        fn equivalent(&self, other: &String) -> bool {
            self == other.as_str()
        }
    }    

    let mut map = TestMap::new();
    map.insert(1, "key".to_string());

    let result = map.swap_remove_full(1, &"key");
    assert!(result.is_some());
    let (index, key, value) = result.unwrap();
    assert_eq!(index, 0);
    assert_eq!(key, "key");
    assert_eq!(value, "key");
}

#[test]
fn test_swap_remove_full_non_existing_entry() {
    struct TestMap {
        entries: Vec<(u32, String)>,
        indices: Vec<u32>,
    }
    
    impl TestMap {
        fn new() -> Self {
            Self { entries: Vec::new(), indices: Vec::new() }
        }

        fn insert(&mut self, hash: u32, key: String) {
            self.entries.push((hash, key));
            self.indices.push(hash);
        }

        fn swap_remove_finish(&mut self, index: usize) -> (String, String) {
            let entry = self.entries.swap_remove(index);
            (entry.1, entry.1)
        }

        fn find_entry(&self, hash: u32, eq: impl Fn(&String) -> bool) -> Result<(usize, u32), ()> {
            for (i, (h, k)) in self.entries.iter().enumerate() {
                if *h == hash && eq(k) {
                    return Ok((i, *h));
                }
            }
            Err(())
        }
    }

    impl Equivalent<String> for str {
        fn equivalent(&self, other: &String) -> bool {
            self == other.as_str()
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "key".to_string());

    let result = map.swap_remove_full(2, &"nonexistent_key");
    assert!(result.is_none());
}


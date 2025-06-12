// Answer 0

#[test]
fn test_entry_increments_existing_key() {
    struct TestHeaderMap {
        map: std::collections::HashMap<String, u32>,
        max_size: usize,
    }

    impl TestHeaderMap {
        fn new(max_size: usize) -> Self {
            TestHeaderMap {
                map: std::collections::HashMap::new(),
                max_size,
            }
        }

        fn entry(&mut self, key: &str) -> &mut u32 {
            if self.map.len() >= self.max_size {
                panic!("size overflows MAX_SIZE");
            }
            self.map.entry(key.to_string()).or_insert(0)
        }

        fn get(&self, key: &str) -> Option<&u32> {
            self.map.get(key)
        }
    }

    let mut map = TestHeaderMap::new(5);
    let headers = &["content-length", "x-hello", "Content-Length", "x-world"];

    for &header in headers {
        let counter = map.entry(header);
        *counter += 1;
    }

    assert_eq!(map.get("content-length"), Some(&2));
    assert_eq!(map.get("x-hello"), Some(&1));
    assert_eq!(map.get("Content-Length"), Some(&2));
}

#[test]
#[should_panic(expected = "size overflows MAX_SIZE")]
fn test_entry_panics_on_max_size() {
    struct TestHeaderMap {
        map: std::collections::HashMap<String, u32>,
        max_size: usize,
    }

    impl TestHeaderMap {
        fn new(max_size: usize) -> Self {
            TestHeaderMap {
                map: std::collections::HashMap::new(),
                max_size,
            }
        }

        fn entry(&mut self, key: &str) -> &mut u32 {
            if self.map.len() >= self.max_size {
                panic!("size overflows MAX_SIZE");
            }
            self.map.entry(key.to_string()).or_insert(0)
        }
    }

    let mut map = TestHeaderMap::new(2);
    map.entry("one");
    map.entry("two");
    map.entry("three"); // This should cause a panic
}


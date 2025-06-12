// Answer 0

#[test]
fn test_iter_empty() {
    struct TestHeaderMap {
        map: std::collections::HashMap<String, Vec<String>>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap {
                map: std::collections::HashMap::new(),
            }
        }

        fn entry(&mut self, key: &str) -> Option<&Vec<String>> {
            self.map.get(key)
        }

        fn insert(&mut self, key: &str, value: String) {
            self.map.entry(key.to_string()).or_insert_with(Vec::new).push(value);
        }
    }

    let mut map = TestHeaderMap::new();
    assert!(map.entry("host").is_none());
}

#[test]
fn test_iter_single_value() {
    struct TestHeaderMap {
        map: std::collections::HashMap<String, Vec<String>>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap {
                map: std::collections::HashMap::new(),
            }
        }

        fn entry(&mut self, key: &str) -> Option<&Vec<String>> {
            self.map.get(key)
        }

        fn insert(&mut self, key: &str, value: String) {
            self.map.entry(key.to_string()).or_insert_with(Vec::new).push(value);
        }

        fn iter(&self, key: &str) -> std::slice::Iter<'_, String> {
            self.map.get(key).map_or_else(|| std::iter::empty().iter(), |v| v.iter())
        }
    }

    let mut map = TestHeaderMap::new();
    map.insert("host", "world".to_string());

    let iter = map.iter("host");
    let values: Vec<_> = iter.collect();
    assert_eq!(values, vec!["world"]);
}

#[test]
fn test_iter_multiple_values() {
    struct TestHeaderMap {
        map: std::collections::HashMap<String, Vec<String>>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap {
                map: std::collections::HashMap::new(),
            }
        }

        fn entry(&mut self, key: &str) -> Option<&Vec<String>> {
            self.map.get(key)
        }

        fn insert(&mut self, key: &str, value: String) {
            self.map.entry(key.to_string()).or_insert_with(Vec::new).push(value);
        }

        fn iter(&self, key: &str) -> std::slice::Iter<'_, String> {
            self.map.get(key).map_or_else(|| std::iter::empty().iter(), |v| v.iter())
        }
    }

    let mut map = TestHeaderMap::new();
    map.insert("host", "world".to_string());
    map.insert("host", "earth".to_string());

    let iter = map.iter("host");
    let values: Vec<_> = iter.collect();
    assert_eq!(values, vec!["world", "earth"]);
}

#[test]
fn test_iter_not_found() {
    struct TestHeaderMap {
        map: std::collections::HashMap<String, Vec<String>>,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap {
                map: std::collections::HashMap::new(),
            }
        }

        fn entry(&mut self, key: &str) -> Option<&Vec<String>> {
            self.map.get(key)
        }

        fn insert(&mut self, key: &str, value: String) {
            self.map.entry(key.to_string()).or_insert_with(Vec::new).push(value);
        }

        fn iter(&self, key: &str) -> std::slice::Iter<'_, String> {
            self.map.get(key).map_or_else(|| std::iter::empty().iter(), |v| v.iter())
        }
    }

    let mut map = TestHeaderMap::new();
    map.insert("host", "world".to_string());
    
    let iter = map.iter("nonexistent");
    let values: Vec<_> = iter.collect();
    assert!(values.is_empty());
}


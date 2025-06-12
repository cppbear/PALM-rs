// Answer 0

#[derive(Debug)]
struct HeaderMap {
    entries: std::collections::HashMap<String, String>,
}

impl HeaderMap {
    fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }

    fn insert(&mut self, key: &str, value: &str) {
        self.entries.insert(key.to_string(), value.to_string());
    }

    fn get2(&self, key: &str) -> Option<&String> {
        self.entries.get(key)
    }

    fn index(&self, index: &str) -> &String {
        match self.get2(index) {
            Some(val) => val,
            None => panic!("no entry found for key {:?}", index),
        }
    }
}

#[test]
fn test_index_existing_key() {
    let mut header_map = HeaderMap::new();
    header_map.insert("Content-Type", "application/json");
    
    let value = header_map.index("Content-Type");
    assert_eq!(value.as_str(), "application/json");
}

#[should_panic(expected = "no entry found for key \"Non-Existing-Key\"")]
#[test]
fn test_index_non_existing_key() {
    let header_map = HeaderMap::new();
    header_map.index("Non-Existing-Key");
}


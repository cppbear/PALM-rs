// Answer 0

#[derive(Debug, PartialEq)]
struct TestMap {
    data: Vec<(i32, String)>,
}

impl TestMap {
    pub fn new() -> Self {
        TestMap { data: Vec::new() }
    }

    pub fn binary_search_keys(&self, key: &i32) -> Result<usize, usize> {
        self.data.binary_search_by_key(key, |(k, _)| *k)
    }

    pub fn insert_before(&mut self, index: usize, key: i32, value: String) -> (usize, Option<String>) {
        self.data.insert(index, (key, value));
        (index, None)
    }

    pub fn insert_sorted(&mut self, key: i32, value: String) -> (usize, Option<String>) {
        match self.binary_search_keys(&key) {
            Ok(i) => (i, Some(std::mem::replace(&mut self.data[i].1, value))),
            Err(i) => self.insert_before(i, key, value),
        }
    }
}

#[test]
fn test_insert_sorted_update_existing() {
    let mut map = TestMap::new();
    map.insert_sorted(1, "one".to_string());
    let result = map.insert_sorted(1, "uno".to_string());
    assert_eq!(result, (0, Some("one".to_string())));
}

#[test]
fn test_insert_sorted_insert_new() {
    let mut map = TestMap::new();
    map.insert_sorted(1, "one".to_string());
    let result = map.insert_sorted(2, "two".to_string());
    assert_eq!(result, (1, None));
    assert_eq!(map.data.len(), 2);
}

#[test]
fn test_insert_sorted_sorted_keys() {
    let mut map = TestMap::new();
    map.insert_sorted(1, "one".to_string());
    map.insert_sorted(3, "three".to_string());
    let result = map.insert_sorted(2, "two".to_string());
    assert_eq!(result, (1, None)); 
    assert_eq!(map.data[1].0, 2);
    assert_eq!(map.data[1].1, "two");
}

#[test]
#[should_panic]
fn test_insert_sorted_panic_on_access() {
    let mut map = TestMap::new();
    map.insert_sorted(1, "one".to_string());
    let _ = map.insert_sorted(1, "uno".to_string());
    // Accessing index out of range to trigger panic.
    let _ = map.data[1]; 
}


// Answer 0

#[derive(Debug)]
struct TestMap {
    data: std::collections::HashMap<i32, i32>,
}

impl TestMap {
    fn new() -> Self {
        TestMap {
            data: std::collections::HashMap::new(),
        }
    }

    fn insert(&mut self, key: i32, value: i32) {
        self.data.insert(key, value);
    }

    fn iter(&self) -> std::collections::hash_map::Iter<'_, i32, i32> {
        self.data.iter()
    }
}

impl std::fmt::Debug for TestMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

#[test]
fn test_empty_map() {
    let map = TestMap::new();
    let result = format!("{:?}", map);
    assert_eq!(result, "{}");
}

#[test]
fn test_single_entry_map() {
    let mut map = TestMap::new();
    map.insert(1, 100);
    let result = format!("{:?}", map);
    assert_eq!(result, "{1: 100}");
}

#[test]
fn test_multiple_entries_map() {
    let mut map = TestMap::new();
    map.insert(1, 100);
    map.insert(2, 200);
    map.insert(3, 300);
    let result = format!("{:?}", map);
    assert_eq!(result, "{1: 100, 2: 200, 3: 300}");
}

#[test]
fn test_duplicate_keys() {
    let mut map = TestMap::new();
    map.insert(1, 100);
    map.insert(1, 200); // This will overwrite the previous value
    let result = format!("{:?}", map);
    assert_eq!(result, "{1: 200}");
}

#[should_panic]
fn test_panicking_condition() {
    // Define a test case that would trigger a panic if any.
    // Currently, the function does not have a clear panic condition.
    // If panic conditions are identified in other parts of the system in the future, 
    // one might add a specific test for those scenarios. 
}


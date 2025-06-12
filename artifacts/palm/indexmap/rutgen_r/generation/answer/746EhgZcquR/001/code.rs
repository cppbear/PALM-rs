// Answer 0

#[derive(Debug)]
struct TestMap {
    entries: Vec<(i32, String)>,
}

impl TestMap {
    fn new() -> Self {
        Self { entries: Vec::new() }
    }

    fn insert(&mut self, key: i32, value: String) {
        self.entries.push((key, value));
    }

    fn swap_remove_entry(&mut self, index: usize) -> (i32, String) {
        let last_index = self.entries.len() - 1;
        self.entries.swap(index, last_index);
        self.entries.pop().unwrap()
    }

    fn swap_remove(&mut self, index: usize) -> String {
        self.swap_remove_entry(index).1
    }
}

#[test]
fn test_swap_remove() {
    let mut map = TestMap::new();
    map.insert(1, "One".to_string());
    map.insert(2, "Two".to_string());
    map.insert(3, "Three".to_string());

    // Test removing the last element
    let value = map.swap_remove(2);
    assert_eq!(value, "Three");

    // Test removing the first element after the swap
    let value = map.swap_remove(0);
    assert_eq!(value, "One");

    // Test removing the remaining element
    let value = map.swap_remove(0);
    assert_eq!(value, "Two");
}

#[test]
#[should_panic]
fn test_swap_remove_out_of_bounds() {
    let mut map = TestMap::new();
    map.insert(1, "One".to_string());
    map.insert(2, "Two".to_string());

    // Attempting to remove an out-of-bounds index should panic
    let _ = map.swap_remove(5);
}


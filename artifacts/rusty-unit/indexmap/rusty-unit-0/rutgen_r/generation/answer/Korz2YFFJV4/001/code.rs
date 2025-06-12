// Answer 0

#[test]
fn test_remove_entry_from_non_empty_map() {
    struct TestMap {
        data: Vec<(i32, String)>,
    }

    impl TestMap {
        pub fn new() -> Self {
            TestMap { data: Vec::new() }
        }

        pub fn insert(&mut self, key: i32, value: String) {
            self.data.push((key, value));
        }

        pub fn swap_remove_entry(&mut self, index: usize) -> (i32, String) {
            let len = self.data.len();
            if index >= len {
                panic!("Index out of bounds");
            }
            self.data.swap_remove(index)
        }

        pub fn remove_entry(self, index: usize) -> (i32, String) {
            self.swap_remove_entry(index)
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    map.insert(3, "three".to_string());
    
    let (key, value) = map.remove_entry(1);
    assert_eq!(key, 2);
    assert_eq!(value, "two".to_string());
    assert_eq!(map.len(), 2);
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_remove_entry_from_empty_map_should_panic() {
    struct TestMap {
        data: Vec<(i32, String)>,
    }

    impl TestMap {
        pub fn new() -> Self {
            TestMap { data: Vec::new() }
        }

        pub fn swap_remove_entry(&mut self, index: usize) -> (i32, String) {
            let len = self.data.len();
            if index >= len {
                panic!("Index out of bounds");
            }
            self.data.swap_remove(index)
        }

        pub fn remove_entry(self, index: usize) -> (i32, String) {
            self.swap_remove_entry(index)
        }
    }

    let map = TestMap::new();
    let _ = map.remove_entry(0); // This should panic
}

#[test]
fn test_remove_entry_with_exact_last_index() {
    struct TestMap {
        data: Vec<(i32, String)>,
    }

    impl TestMap {
        pub fn new() -> Self {
            TestMap { data: Vec::new() }
        }

        pub fn insert(&mut self, key: i32, value: String) {
            self.data.push((key, value));
        }

        pub fn swap_remove_entry(&mut self, index: usize) -> (i32, String) {
            let len = self.data.len();
            if index >= len {
                panic!("Index out of bounds");
            }
            self.data.swap_remove(index)
        }

        pub fn remove_entry(self, index: usize) -> (i32, String) {
            self.swap_remove_entry(index)
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    
    // remove last entry
    let (key, value) = map.remove_entry(1);
    assert_eq!(key, 2);
    assert_eq!(value, "two".to_string());
    assert_eq!(map.len(), 1);
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_remove_entry_with_invalid_high_index() {
    struct TestMap {
        data: Vec<(i32, String)>,
    }

    impl TestMap {
        pub fn new() -> Self {
            TestMap { data: Vec::new() }
        }

        pub fn insert(&mut self, key: i32, value: String) {
            self.data.push((key, value));
        }

        pub fn swap_remove_entry(&mut self, index: usize) -> (i32, String) {
            let len = self.data.len();
            if index >= len {
                panic!("Index out of bounds");
            }
            self.data.swap_remove(index)
        }

        pub fn remove_entry(self, index: usize) -> (i32, String) {
            self.swap_remove_entry(index)
        }
    }

    let mut map = TestMap::new();
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    
    let _ = map.remove_entry(3); // This should panic
}


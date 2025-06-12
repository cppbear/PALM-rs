// Answer 0

#[test]
fn test_get_index_valid() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        pub fn new() -> Self {
            TestMap { entries: Vec::new() }
        }

        pub fn insert(&mut self, key: i32, value: i32) {
            self.entries.push((key, value));
        }

        pub fn len(&self) -> usize {
            self.entries.len()
        }

        pub fn as_entries(&self) -> &Vec<(i32, i32)> {
            &self.entries
        }
    }

    let mut map = TestMap::new();
    map.insert(1, 100);
    map.insert(2, 200);
    
    assert_eq!(map.get_index(0), Some(&(1, 100)));
    assert_eq!(map.get_index(1), Some(&(2, 200)));
}

#[test]
fn test_get_index_out_of_bounds() {
    struct TestMap {
        entries: Vec<(i32, i32)>,
    }

    impl TestMap {
        pub fn new() -> Self {
            TestMap { entries: Vec::new() }
        }

        pub fn insert(&mut self, key: i32, value: i32) {
            self.entries.push((key, value));
        }

        pub fn len(&self) -> usize {
            self.entries.len()
        }

        pub fn as_entries(&self) -> &Vec<(i32, i32)> {
            &self.entries
        }
    }

    let mut map = TestMap::new();
    map.insert(1, 100);

    assert_eq!(map.get_index(1), None);
    assert_eq!(map.get_index(2), None);
}


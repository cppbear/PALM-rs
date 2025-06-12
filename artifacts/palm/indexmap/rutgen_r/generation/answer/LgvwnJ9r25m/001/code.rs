// Answer 0

#[test]
fn test_shift_remove_non_empty_entry() {
    struct TestMap {
        data: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { data: Vec::new() }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.data.push((key, value));
        }

        fn shift_remove_entry(&mut self, index: usize) -> (i32, String) {
            if index >= self.data.len() {
                panic!("Index out of bounds");
            }
            let (key, value) = self.data.remove(index);
            (key, value)
        }

        fn shift_remove(&mut self, index: usize) -> String {
            self.shift_remove_entry(index).1
        }
    }

    let mut map = TestMap::new();
    map.insert(0, "zero".to_string());
    map.insert(1, "one".to_string());
    map.insert(2, "two".to_string());
    
    let result = map.shift_remove(1);
    assert_eq!(result, "one");
    assert_eq!(map.data.len(), 2);
    assert_eq!(map.data[0].1, "zero");
    assert_eq!(map.data[1].1, "two");
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_shift_remove_panic_on_empty() {
    struct TestMap {
        data: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { data: Vec::new() }
        }

        fn shift_remove_entry(&mut self, index: usize) -> (i32, String) {
            if index >= self.data.len() {
                panic!("Index out of bounds");
            }
            let (key, value) = self.data.remove(index);
            (key, value)
        }

        fn shift_remove(&mut self, index: usize) -> String {
            self.shift_remove_entry(index).1
        }
    }

    let mut map = TestMap::new();
    map.shift_remove(0);
}

#[test]
#[should_panic(expected = "Index out of bounds")]
fn test_shift_remove_panic_on_invalid_index() {
    struct TestMap {
        data: Vec<(i32, String)>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap { data: Vec::new() }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.data.push((key, value));
        }

        fn shift_remove_entry(&mut self, index: usize) -> (i32, String) {
            if index >= self.data.len() {
                panic!("Index out of bounds");
            }
            let (key, value) = self.data.remove(index);
            (key, value)
        }

        fn shift_remove(&mut self, index: usize) -> String {
            self.shift_remove_entry(index).1
        }
    }

    let mut map = TestMap::new();
    map.insert(0, "zero".to_string());
    
    map.shift_remove(1);
}


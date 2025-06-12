// Answer 0

#[test]
fn test_shift_remove_existing_key() {
    struct TestMap {
        data: IndexMap<i32, String, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            let mut data = IndexMap::new();
            data.insert(1, String::from("one"));
            data.insert(2, String::from("two"));
            data.insert(3, String::from("three"));
            TestMap { data }
        }

        fn shift_remove(&mut self, key: &i32) -> Option<String> {
            self.data.shift_remove(key)
        }

        fn contains(&self, key: &i32) -> bool {
            self.data.contains_key(key)
        }
    }

    let mut test_map = TestMap::new();
    assert_eq!(test_map.shift_remove(&2), Some(String::from("two")));
    assert!(!test_map.contains(&2));
}

#[test]
fn test_shift_remove_non_existing_key() {
    struct TestMap {
        data: IndexMap<i32, String, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            let mut data = IndexMap::new();
            data.insert(1, String::from("one"));
            data.insert(2, String::from("two"));
            data.insert(3, String::from("three"));
            TestMap { data }
        }

        fn shift_remove(&mut self, key: &i32) -> Option<String> {
            self.data.shift_remove(key)
        }
    }

    let mut test_map = TestMap::new();
    assert_eq!(test_map.shift_remove(&4), None);
}

#[test]
fn test_shift_remove_first_element() {
    struct TestMap {
        data: IndexMap<i32, String, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            let mut data = IndexMap::new();
            data.insert(1, String::from("one"));
            data.insert(2, String::from("two"));
            TestMap { data }
        }

        fn shift_remove(&mut self, key: &i32) -> Option<String> {
            self.data.shift_remove(key)
        }

        fn get_value(&self, key: &i32) -> Option<&String> {
            self.data.get(key)
        }
    }

    let mut test_map = TestMap::new();
    assert_eq!(test_map.shift_remove(&1), Some(String::from("one")));
    assert_eq!(test_map.get_value(&2), Some(&String::from("two")));
}

#[test]
fn test_shift_remove_last_element() {
    struct TestMap {
        data: IndexMap<i32, String, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            let mut data = IndexMap::new();
            data.insert(1, String::from("one"));
            data.insert(2, String::from("two"));
            TestMap { data }
        }

        fn shift_remove(&mut self, key: &i32) -> Option<String> {
            self.data.shift_remove(key)
        }

        fn get_value(&self, key: &i32) -> Option<&String> {
            self.data.get(key)
        }
    }

    let mut test_map = TestMap::new();
    assert_eq!(test_map.shift_remove(&2), Some(String::from("two")));
    assert!(test_map.get_value(&2).is_none());
}


// Answer 0

#[test]
fn test_values_mut() {
    struct TestMap {
        map: indexmap::IndexMap<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                map: indexmap::IndexMap::new(),
            }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.map.insert(key, value);
        }

        fn values_mut(&mut self) -> indexmap::map::ValuesMut<'_, i32, String> {
            self.map.values_mut()
        }
    }

    let mut test_map = TestMap::new();
    test_map.insert(1, "one".to_string());
    test_map.insert(2, "two".to_string());

    let mut values_iter = test_map.values_mut();
    if let Some(value) = values_iter.next() {
        *value = "changed".to_string();
    }

    assert_eq!(test_map.map.get(&1), Some(&"changed".to_string()));
    assert_eq!(test_map.map.get(&2), Some(&"two".to_string()));
}

#[test]
fn test_values_mut_empty() {
    struct TestMap {
        map: indexmap::IndexMap<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                map: indexmap::IndexMap::new(),
            }
        }

        fn values_mut(&mut self) -> indexmap::map::ValuesMut<'_, i32, String> {
            self.map.values_mut()
        }
    }

    let mut test_map = TestMap::new();
    let mut values_iter = test_map.values_mut();
    assert!(values_iter.next().is_none());
}

#[test]
fn test_values_mut_boundary() {
    struct TestMap {
        map: indexmap::IndexMap<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                map: indexmap::IndexMap::new(),
            }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.map.insert(key, value);
        }

        fn values_mut(&mut self) -> indexmap::map::ValuesMut<'_, i32, String> {
            self.map.values_mut()
        }
    }

    let mut test_map = TestMap::new();
    test_map.insert(1, "one".to_string());
    test_map.insert(2, "two".to_string());
    test_map.insert(3, "three".to_string());

    {
        let mut values_iter = test_map.values_mut();
        values_iter.next().map(|v| *v = "first".to_string());
        values_iter.next().map(|v| *v = "second".to_string());
        values_iter.next().map(|v| *v = "third".to_string());
    }

    assert_eq!(test_map.map.get(&1), Some(&"first".to_string()));
    assert_eq!(test_map.map.get(&2), Some(&"second".to_string()));
    assert_eq!(test_map.map.get(&3), Some(&"third".to_string()));
}


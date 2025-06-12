// Answer 0

#[test]
fn test_insert() {
    struct TestMap {
        map: std::collections::HashMap<i32, i32>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                map: std::collections::HashMap::new(),
            }
        }

        fn insert(&mut self, key: i32, value: i32) -> &mut i32 {
            self.map.insert(key, value);
            self.map.get_mut(&key).unwrap()
        }
    }

    let mut test_map = TestMap::new();
    let value_ref = test_map.insert(1, 10);
    *value_ref += 5;

    assert_eq!(test_map.map.get(&1), Some(&15));
}

#[test]
fn test_insert_multiple() {
    struct TestMap {
        map: std::collections::HashMap<i32, i32>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                map: std::collections::HashMap::new(),
            }
        }

        fn insert(&mut self, key: i32, value: i32) -> &mut i32 {
            self.map.insert(key, value);
            self.map.get_mut(&key).unwrap()
        }
    }

    let mut test_map = TestMap::new();
    let value_ref1 = test_map.insert(1, 10);
    *value_ref1 += 5;

    let value_ref2 = test_map.insert(2, 20);
    *value_ref2 += 3;

    assert_eq!(test_map.map.get(&1), Some(&15));
    assert_eq!(test_map.map.get(&2), Some(&23));
}

#[test]
fn test_insert_overwrite() {
    struct TestMap {
        map: std::collections::HashMap<i32, i32>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                map: std::collections::HashMap::new(),
            }
        }

        fn insert(&mut self, key: i32, value: i32) -> &mut i32 {
            self.map.insert(key, value);
            self.map.get_mut(&key).unwrap()
        }
    }

    let mut test_map = TestMap::new();
    let value_ref = test_map.insert(1, 10);
    *value_ref += 5;

    let value_ref_overwrite = test_map.insert(1, 20);
    *value_ref_overwrite += 10;

    assert_eq!(test_map.map.get(&1), Some(&30));
}


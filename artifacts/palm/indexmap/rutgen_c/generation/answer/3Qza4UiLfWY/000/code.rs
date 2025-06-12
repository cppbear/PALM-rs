// Answer 0

#[test]
fn test_index_mut_found() {
    struct TestMap {
        map: IndexMap<i32, String, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            let mut map = IndexMap::new();
            map.insert(1, "value1".to_string());
            map.insert(2, "value2".to_string());
            TestMap { map }
        }

        fn index_mut(&mut self, key: &i32) -> &mut String {
            self.map.index_mut(*key)
        }
    }

    let mut test_map = TestMap::new();
    let value = test_map.index_mut(&1);
    *value = "updated_value1".to_string();
    assert_eq!(test_map.map.get(&1), Some(&"updated_value1".to_string()));
}

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_mut_not_found() {
    struct TestMap {
        map: IndexMap<i32, String, RandomState>,
    }

    impl TestMap {
        fn new() -> Self {
            let mut map = IndexMap::new();
            map.insert(1, "value1".to_string());
            TestMap { map }
        }

        fn index_mut(&mut self, key: &i32) -> &mut String {
            self.map.index_mut(*key)
        }
    }

    let mut test_map = TestMap::new();
    test_map.index_mut(&2); // This should panic as key 2 is not present.
}


// Answer 0

#[test]
fn test_get_full_found() {
    struct TestKeys;
    struct TestValues;

    impl Hash for TestKeys {
        fn hash<H: Hasher>(&self, _state: &mut H) {}
    }

    impl PartialEq for TestKeys {
        fn eq(&self, _other: &Self) -> bool {
            true
        }
    }

    impl Equivalent<TestKeys> for TestKeys {
        fn equivalent(&self, _: &TestKeys) -> bool {
            true
        }
    }

    let mut index_map = IndexMap::<TestKeys, TestValues, RandomState>::default();
    let key = TestKeys;
    let value = TestValues;

    index_map.insert(key, value);

    if let Some((index, k, v)) = index_map.get_full(&key) {
        assert_eq!(index, 0);
        assert!(std::ptr::addr_of!(*k) == std::ptr::addr_of!(key));
        assert!(std::ptr::addr_of!(*v) == std::ptr::addr_of!(value));
    } else {
        panic!("Expected to find the entry in the map.");
    }
}


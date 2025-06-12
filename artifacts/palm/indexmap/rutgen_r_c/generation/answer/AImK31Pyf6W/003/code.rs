// Answer 0

#[test]
fn test_shift_remove_full_single_entry_non_equivalent() {
    struct TestKeys;
    struct TestValues;
    struct TestBuildHasher;

    impl Hash for TestKeys {
        fn hash<H: Hasher>(&self, _state: &mut H) {}
    }

    impl PartialEq for TestKeys {
        fn eq(&self, _other: &Self) -> bool {
            false
        }
    }

    impl Equivalent<TestKeys> for TestKeys {
        fn equivalent(&self, _other: &TestKeys) -> bool {
            false
        }
    }

    impl TestValues {
        fn new() -> Self {
            TestValues {}
        }
    }

    let mut index_map = IndexMap::<TestKeys, TestValues, TestBuildHasher>::new();
    index_map.insert(TestKeys {}, TestValues::new());
    
    let result = index_map.shift_remove_full(&TestKeys {});
    assert!(result.is_none());
}

#[test]
fn test_shift_remove_full_empty_map() {
    struct TestKeys;
    struct TestValues;
    struct TestBuildHasher;

    impl Hash for TestKeys {
        fn hash<H: Hasher>(&self, _state: &mut H) {}
    }

    impl PartialEq for TestKeys {
        fn eq(&self, _other: &Self) -> bool {
            false
        }
    }

    impl Equivalent<TestKeys> for TestKeys {
        fn equivalent(&self, _other: &TestKeys) -> bool {
            false
        }
    }

    let mut index_map: IndexMap<TestKeys, TestValues, TestBuildHasher> = IndexMap::new();

    let result = index_map.shift_remove_full(&TestKeys {});
    assert!(result.is_none());
}


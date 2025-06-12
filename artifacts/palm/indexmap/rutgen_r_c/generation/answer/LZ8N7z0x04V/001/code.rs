// Answer 0

#[test]
fn test_index_valid_key() {
    struct TestKeys;
    struct TestValues;

    impl Hash for TestKeys {
        fn hash<H: Hasher>(&self, _state: &mut H) {}
    }

    impl PartialEq for TestKeys {
        fn eq(&self, _: &Self) -> bool { true }
    }

    impl Eq for TestKeys {}

    let mut map = IndexMap::<TestKeys, TestValues, RandomState::new>();
    let key = TestKeys;
    let value = TestValues;

    map.insert(key, value);
    let result = map.index(&key);
    assert_eq!(result, &value);
}

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_invalid_key() {
    struct TestKeys;
    struct TestValues;

    impl Hash for TestKeys {
        fn hash<H: Hasher>(&self, _state: &mut H) {}
    }

    impl PartialEq for TestKeys {
        fn eq(&self, _: &Self) -> bool { true }
    }

    impl Eq for TestKeys {}

    let map = IndexMap::<TestKeys, TestValues, RandomState::new>();
    let missing_key = TestKeys;

    // This should panic because the key does not exist.
    let _result = map.index(&missing_key);
}


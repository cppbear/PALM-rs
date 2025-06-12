// Answer 0

#[derive(Hash, Eq, PartialEq, Debug)]
struct TestKey(usize);
#[derive(Debug)]
struct TestValue(String);

struct TestMap {
    map: indexmap::IndexMap<TestKey, TestValue>,
}

impl TestMap {
    fn new() -> Self {
        TestMap {
            map: indexmap::IndexMap::new(),
        }
    }

    fn insert(&mut self, key: TestKey, value: TestValue) {
        self.map.insert(key, value);
    }

    fn shift_remove_entry(&mut self, key: &TestKey) -> Option<(TestKey, TestValue)> {
        match self.map.shift_remove_entry(key) {
            Some((key, value)) => Some((key, value)),
            None => None,
        }
    }
}

#[test]
fn test_shift_remove_entry_existing_key() {
    let mut test_map = TestMap::new();
    let key = TestKey(1);
    let value = TestValue("value1".to_string());

    test_map.insert(key.clone(), value);

    let result = test_map.shift_remove_entry(&key);
    assert_eq!(result, Some((key.clone(), TestValue("value1".to_string()))));
}

#[test]
fn test_shift_remove_entry_non_existing_key() {
    let mut test_map = TestMap::new();
    let key = TestKey(2); // Key that was not inserted

    let result = test_map.shift_remove_entry(&key);
    assert_eq!(result, None);
}


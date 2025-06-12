// Answer 0

#[derive(Debug)]
struct TestKey(String);

#[derive(Debug)]
struct TestValue(i32);

impl Hash for TestKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl PartialEq for TestKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for TestKey {}

impl Equivalent<TestKey> for TestKey {
    fn equivalent(&self, other: &TestKey) -> bool {
        self.eq(other)
    }
}

#[test]
fn test_swap_remove_existing_key() {
    let mut map = IndexMap::<TestKey, TestValue, RandomState>::new();
    map.insert(TestKey("key1".into()), TestValue(1));
    let _ = map.swap_remove(&TestKey("key1".into()));
}

#[test]
fn test_swap_remove_non_existing_key() {
    let mut map = IndexMap::<TestKey, TestValue, RandomState>::new();
    map.insert(TestKey("key1".into()), TestValue(1));
    let _ = map.swap_remove(&TestKey("key2".into()));
}

#[test]
fn test_swap_remove_with_multiple_keys() {
    let mut map = IndexMap::<TestKey, TestValue, RandomState>::new();
    map.insert(TestKey("key1".into()), TestValue(1));
    map.insert(TestKey("key2".into()), TestValue(2));
    map.insert(TestKey("key3".into()), TestValue(3));
    let _ = map.swap_remove(&TestKey("key2".into()));
}

#[test]
fn test_swap_remove_empty_key() {
    let mut map = IndexMap::<TestKey, TestValue, RandomState>::new();
    map.insert(TestKey("".into()), TestValue(0));
    let _ = map.swap_remove(&TestKey("".into()));
}

#[test]
fn test_swap_remove_after_deletion() {
    let mut map = IndexMap::<TestKey, TestValue, RandomState>::new();
    map.insert(TestKey("key1".into()), TestValue(1));
    let _ = map.swap_remove(&TestKey("key1".into()));
    let _ = map.swap_remove(&TestKey("key1".into())); // Attempting to remove again
}

#[test]
fn test_swap_remove_on_empty_map() {
    let mut map = IndexMap::<TestKey, TestValue, RandomState>::new();
    let _ = map.swap_remove(&TestKey("key1".into())); // Nothing in the map to remove
}

#[test]
fn test_swap_remove_with_large_key() {
    let large_key = TestKey("key".repeat(1000)); // Large key
    let mut map = IndexMap::<TestKey, TestValue, RandomState>::new();
    map.insert(large_key.clone(), TestValue(1));
    let _ = map.swap_remove(&large_key);
}


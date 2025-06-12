// Answer 0

#[derive(Debug)]
struct MockEntry<K, V> {
    key: K,
    value: V,
}

trait Entry<K, V> {
    fn swap_remove_entry(self) -> (K, V);
}

impl<K, V> Entry<K, V> for MockEntry<K, V> {
    fn swap_remove_entry(self) -> (K, V) {
        (self.key, self.value)
    }
}

#[test]
fn test_remove_entry() {
    let entry = MockEntry { key: "test_key", value: "test_value" };
    let (key, value) = entry.remove_entry();
    assert_eq!(key, "test_key");
    assert_eq!(value, "test_value");
}

#[test]
fn test_remove_entry_with_different_types() {
    let entry = MockEntry { key: 42, value: 3.14 };
    let (key, value) = entry.remove_entry();
    assert_eq!(key, 42);
    assert_eq!(value, 3.14);
}


// Answer 0

#[derive(Debug)]
struct KeyValue<K, V> {
    key: K,
    value: V,
}

impl<K, V> KeyValue<K, V> {
    fn key_value(self) -> (K, V) {
        (self.key, self.value)
    }
}

#[test]
fn test_key_value() {
    let kv = KeyValue { key: 1, value: "one" };
    let (key, value) = kv.key_value();
    assert_eq!(key, 1);
    assert_eq!(value, "one");
}

#[test]
fn test_key_value_different_types() {
    let kv = KeyValue { key: "two", value: 2.0 };
    let (key, value) = kv.key_value();
    assert_eq!(key, "two");
    assert_eq!(value, 2.0);
}


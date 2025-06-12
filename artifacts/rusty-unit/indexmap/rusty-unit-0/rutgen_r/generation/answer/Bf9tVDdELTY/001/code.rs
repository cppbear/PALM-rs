// Answer 0

#[test]
fn test_ref_mut() {
    struct KeyValue<K, V> {
        key: K,
        value: V,
    }

    impl KeyValue<String, i32> {
        fn new(key: String, value: i32) -> Self {
            KeyValue { key, value }
        }

        fn ref_mut(&mut self) -> (&K, &mut V) {
            (&self.key, &mut self.value)
        }
    }

    let mut kv = KeyValue::new("test".to_string(), 42);

    let (key_ref, value_ref) = kv.ref_mut();
    assert_eq!(key_ref, &"test".to_string());
    *value_ref += 1; // Mutating value through a mutable reference
    assert_eq!(kv.value, 43);
}


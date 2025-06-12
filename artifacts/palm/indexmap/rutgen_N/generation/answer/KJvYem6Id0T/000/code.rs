// Answer 0

#[derive(Debug)]
struct KeyValue<K, V> {
    key: K,
    value: V,
}

trait MutableKey<K> {
    fn key_mut(&mut self) -> &mut K;
}

impl<K, V> MutableKey<K> for KeyValue<K, V> {
    fn key_mut(&mut self) -> &mut K {
        &mut self.key
    }
}

#[test]
fn test_key_mut() {
    let mut key_value = KeyValue { key: 42, value: "value" };

    {
        let key_ref = key_value.key_mut();
        *key_ref += 1; // Modify the key
    }

    assert_eq!(key_value.key, 43);
}

#[test]
fn test_key_mut_boundary() {
    let mut key_value = KeyValue { key: i32::MIN, value: "value" };

    {
        let key_ref = key_value.key_mut();
        *key_ref = i32::MAX; // Set key to boundary value
    }

    assert_eq!(key_value.key, i32::MAX);
}


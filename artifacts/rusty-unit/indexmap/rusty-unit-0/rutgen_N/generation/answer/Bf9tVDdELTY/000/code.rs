// Answer 0

#[derive(Debug)]
struct KeyValue<K, V> {
    key: K,
    value: V,
}

impl<K, V> KeyValue<K, V> {
    fn ref_mut(&mut self) -> (&K, &mut V) {
        (&self.key, &mut self.value)
    }
}

#[test]
fn test_ref_mut() {
    let mut kv = KeyValue { key: "test", value: 42 };
    let (key, value) = kv.ref_mut();
    
    assert_eq!(*key, "test");
    *value += 1; // Modify the value through the mutable reference
    assert_eq!(kv.value, 43);
}

#[test]
fn test_ref_mut_multiple_calls() {
    let mut kv = KeyValue { key: "example", value: 100 };
    
    {
        let (_, value1) = kv.ref_mut();
        *value1 += 50;
    }
    
    {
        let (_, value2) = kv.ref_mut();
        assert_eq!(*value2, 150);
        *value2 -= 30;
    }

    assert_eq!(kv.value, 120);
}


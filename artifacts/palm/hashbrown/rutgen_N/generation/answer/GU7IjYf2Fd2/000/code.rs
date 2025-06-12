// Answer 0

#[derive(Debug)]
struct Entry<K, V> {
    key: K,
    value: V,
}

impl<K, V> Entry<K, V> {
    fn key(&self) -> &K {
        &self.key
    }

    fn get(&self) -> &V {
        &self.value
    }
}

struct OccupiedError<K, V> {
    entry: Entry<K, V>,
    value: V,
}

impl<K: std::fmt::Debug, V: std::fmt::Debug> std::fmt::Debug for OccupiedError<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OccupiedError")
            .field("key", self.entry.key())
            .field("old_value", self.entry.get())
            .field("new_value", &self.value)
            .finish()
    }
}

#[test]
fn test_occupied_error_debug() {
    let entry = Entry { key: "test_key", value: "old_value" };
    let occupied_error = OccupiedError { entry, value: "new_value" };

    let expected_output = "OccupiedError { key: \"test_key\", old_value: \"old_value\", new_value: \"new_value\" }";
    
    let result = format!("{:?}", occupied_error);
    assert_eq!(result, expected_output);
}


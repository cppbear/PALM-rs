// Answer 0

#[derive(Debug)]
struct TestEntry<K, V> {
    key: K,
    value: V,
}

impl<K, V> TestEntry<K, V> {
    fn shift_remove_entry(self) -> (K, V) {
        (self.key, self.value)
    }
}

#[test]
fn test_shift_remove() {
    let entry = TestEntry {
        key: 1,
        value: "value_1",
    };
    let value = entry.shift_remove();
    assert_eq!(value, "value_1");
}

#[test]
fn test_shift_remove_boundary() {
    let entry = TestEntry {
        key: 0,
        value: "boundary_value",
    };
    let value = entry.shift_remove();
    assert_eq!(value, "boundary_value");
}

#[test]
#[should_panic]
fn test_shift_remove_empty() {
    let entry: Option<TestEntry<i32, &str>> = None;

    if entry.is_none() {
        panic!("Attempted to shift remove from an empty entry");
    }
}


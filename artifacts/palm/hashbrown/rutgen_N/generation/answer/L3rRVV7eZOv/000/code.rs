// Answer 0

#[derive(Debug)]
struct RawOccupiedEntryMut<'a, K: std::fmt::Debug, V: std::fmt::Debug> {
    key: K,
    value: V,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a, K: std::fmt::Debug, V: std::fmt::Debug> RawOccupiedEntryMut<'a, K, V> {
    fn new(key: K, value: V) -> Self {
        RawOccupiedEntryMut {
            key,
            value,
            _marker: std::marker::PhantomData,
        }
    }

    fn key(&self) -> &K {
        &self.key
    }

    fn get(&self) -> &V {
        &self.value
    }
}

use std::fmt;

impl<'a, K: std::fmt::Debug, V: std::fmt::Debug> fmt::Debug for RawOccupiedEntryMut<'a, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawOccupiedEntryMut")
            .field("key", self.key())
            .field("value", self.get())
            .finish()
    }
}

#[test]
fn test_fmt_debug() {
    let entry = RawOccupiedEntryMut::new("test_key", 42);
    let fmt_output = format!("{:?}", entry);
    assert!(fmt_output.contains("key: \"test_key\""));
    assert!(fmt_output.contains("value: 42"));
}

#[test]
fn test_fmt_debug_with_different_types() {
    let entry = RawOccupiedEntryMut::new(100, "value_string");
    let fmt_output = format!("{:?}", entry);
    assert!(fmt_output.contains("key: 100"));
    assert!(fmt_output.contains("value: \"value_string\""));
}


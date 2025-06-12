// Answer 0

#[test]
fn test_fmt_with_valid_data() {
    use std::fmt;

    struct IndexedEntry<K, V> {
        index: usize,
        key: K,
        value: V,
    }

    impl<K: fmt::Debug, V: fmt::Debug> IndexedEntry<K, V> {
        fn key(&self) -> &K {
            &self.key
        }

        fn get(&self) -> &V {
            &self.value
        }

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("IndexedEntry")
                .field("index", &self.index)
                .field("key", self.key())
                .field("value", self.get())
                .finish()
        }
    }

    let entry = IndexedEntry {
        index: 1,
        key: "test_key",
        value: "test_value",
    };

    let result = format!("{:?}", entry.fmt(&mut fmt::Formatter::new()));
    assert!(result.contains("IndexedEntry"));
    assert!(result.contains("index: 1"));
    assert!(result.contains("key: \"test_key\""));
    assert!(result.contains("value: \"test_value\""));
}

#[test]
fn test_fmt_with_empty_data() {
    use std::fmt;

    struct IndexedEntry<K, V> {
        index: usize,
        key: K,
        value: V,
    }

    impl<K: fmt::Debug, V: fmt::Debug> IndexedEntry<K, V> {
        fn key(&self) -> &K {
            &self.key
        }

        fn get(&self) -> &V {
            &self.value
        }

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("IndexedEntry")
                .field("index", &self.index)
                .field("key", self.key())
                .field("value", self.get())
                .finish()
        }
    }

    let entry = IndexedEntry {
        index: 0,
        key: "",
        value: "",
    };

    let result = format!("{:?}", entry.fmt(&mut fmt::Formatter::new()));
    assert!(result.contains("IndexedEntry"));
    assert!(result.contains("index: 0"));
    assert!(result.contains("key: \"\""));
    assert!(result.contains("value: \"\""));
}


// Answer 0

#[test]
fn test_raw_occupied_entry_mut_fmt() {
    struct RawOccupiedEntryMut<K, V> {
        key: K,
        value: V,
    }

    impl<K, V> RawOccupiedEntryMut<K, V> {
        fn key(&self) -> &K {
            &self.key
        }

        fn get(&self) -> &V {
            &self.value
        }
    }

    let entry = RawOccupiedEntryMut {
        key: "test_key",
        value: 42,
    };

    let mut buffer = String::new();
    let result = std::fmt::write(&mut buffer, |f| {
        entry.fmt(f)
    });

    assert!(result.is_ok());
    assert!(buffer.contains("key: \"test_key\""));
    assert!(buffer.contains("value: 42"));
}

#[test]
fn test_raw_occupied_entry_mut_fmt_empty() {
    struct RawOccupiedEntryMut<K, V> {
        key: K,
        value: V,
    }

    impl<K, V> RawOccupiedEntryMut<K, V> {
        fn key(&self) -> &K {
            &self.key
        }

        fn get(&self) -> &V {
            &self.value
        }
    }

    let entry = RawOccupiedEntryMut {
        key: "",
        value: "",
    };

    let mut buffer = String::new();
    let result = std::fmt::write(&mut buffer, |f| {
        entry.fmt(f)
    });

    assert!(result.is_ok());
    assert!(buffer.contains("key: \"\""));
    assert!(buffer.contains("value: \"\""));
}

#[test]
fn test_raw_occupied_entry_mut_fmt_complex_value() {
    struct RawOccupiedEntryMut<K, V> {
        key: K,
        value: V,
    }

    impl<K, V> RawOccupiedEntryMut<K, V> {
        fn key(&self) -> &K {
            &self.key
        }

        fn get(&self) -> &V {
            &self.value
        }
    }

    let entry = RawOccupiedEntryMut {
        key: "complex_key",
        value: vec![1, 2, 3],
    };

    let mut buffer = String::new();
    let result = std::fmt::write(&mut buffer, |f| {
        entry.fmt(f)
    });

    assert!(result.is_ok());
    assert!(buffer.contains("key: \"complex_key\""));
    assert!(buffer.contains("value: [1, 2, 3]"));
}


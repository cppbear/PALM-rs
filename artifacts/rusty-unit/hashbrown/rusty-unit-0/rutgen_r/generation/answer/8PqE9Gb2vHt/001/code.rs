// Answer 0

#[test]
fn test_fmt_occupied_entry() {
    struct OccupiedEntry<'a, K, V> {
        key: K,
        value: V,
        _marker: std::marker::PhantomData<&'a ()>,
    }

    impl<'a, K: std::fmt::Debug, V: std::fmt::Debug> OccupiedEntry<'a, K, V> {
        fn key(&self) -> &K {
            &self.key
        }

        fn get(&self) -> &V {
            &self.value
        }

        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("OccupiedEntry")
                .field("key", self.key())
                .field("value", self.get())
                .finish()
        }
    }

    let entry = OccupiedEntry {
        key: "test_key",
        value: 42,
        _marker: std::marker::PhantomData,
    };

    let result = entry.fmt(&mut std::fmt::Formatter::new());
    
    assert!(result.is_ok());
}

#[test]
fn test_fmt_occupied_entry_with_different_types() {
    struct OccupiedEntry<'a, K, V> {
        key: K,
        value: V,
        _marker: std::marker::PhantomData<&'a ()>,
    }

    impl<'a, K: std::fmt::Debug, V: std::fmt::Debug> OccupiedEntry<'a, K, V> {
        fn key(&self) -> &K {
            &self.key
        }

        fn get(&self) -> &V {
            &self.value
        }

        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("OccupiedEntry")
                .field("key", self.key())
                .field("value", self.get())
                .finish()
        }
    }

    let entry = OccupiedEntry {
        key: 100,
        value: "test_value",
        _marker: std::marker::PhantomData,
    };

    let result = entry.fmt(&mut std::fmt::Formatter::new());
    
    assert!(result.is_ok());
}


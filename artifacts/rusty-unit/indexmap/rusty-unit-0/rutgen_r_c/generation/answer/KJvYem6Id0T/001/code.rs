// Answer 0

#[test]
fn test_key_mut() {
    struct TestEntries<K, V> {
        inner: Vec<(K, V)>,
    }

    impl<K, V> TestEntries<K, V> {
        fn new() -> Self {
            Self { inner: Vec::new() }
        }
        
        fn insert(&mut self, key: K, value: V) {
            self.inner.push((key, value));
        }
        
        fn get_key_mut(&mut self, index: usize) -> &mut K {
            &mut self.inner[index].0
        }
        
        fn len(&self) -> usize {
            self.inner.len()
        }
    }

    struct TestOccupiedEntry<'a, K, V> {
        entries: &'a mut TestEntries<K, V>,
        index: usize,
    }

    impl<'a, K, V> TestOccupiedEntry<'a, K, V> {
        fn new(entries: &'a mut TestEntries<K, V>, index: usize) -> Self {
            Self { entries, index }
        }

        fn key_mut(&mut self) -> &mut K {
            self.entries.get_key_mut(self.index)
        }
        
        fn index(&self) -> usize {
            self.index
        }
    }

    let mut entries = TestEntries::new();
    entries.insert("key1", "value1");

    // Create an occupied entry for the first index.
    let mut occupied_entry = TestOccupiedEntry::new(&mut entries, 0);

    // Test that we can get a mutable reference to the key and modify it.
    let key_ref = occupied_entry.key_mut();
    *key_ref = "new_key1"; // Modify the key to a new value.

    // Verify that the key in entries has been updated.
    assert_eq!(entries.inner[0].0, "new_key1");
}

#[test]
fn test_key_mut_empty() {
    struct TestEntries<K, V> {
        inner: Vec<(K, V)>,
    }

    impl<K, V> TestEntries<K, V> {
        fn new() -> Self {
            Self { inner: Vec::new() }
        }
        
        fn insert(&mut self, key: K, value: V) {
            self.inner.push((key, value));
        }
        
        fn get_key_mut(&mut self, index: usize) -> &mut K {
            &mut self.inner[index].0
        }
        
        fn len(&self) -> usize {
            self.inner.len()
        }
    }

    struct TestOccupiedEntry<'a, K, V> {
        entries: &'a mut TestEntries<K, V>,
        index: usize,
    }

    impl<'a, K, V> TestOccupiedEntry<'a, K, V> {
        fn new(entries: &'a mut TestEntries<K, V>, index: usize) -> Self {
            Self { entries, index }
        }

        fn key_mut(&mut self) -> &mut K {
            self.entries.get_key_mut(self.index)
        }
        
        fn index(&self) -> usize {
            self.index
        }
    }

    let mut entries = TestEntries::new();

    // Trying to create an occupied entry where there are no elements should panic.
    let result = std::panic::catch_unwind(|| {
        let _occupied_entry = TestOccupiedEntry::new(&mut entries, 0);
        // Attempting to access key_mut() should panic since there is no element.
        let _key_ref = entries.get_key_mut(0);
    });

    assert!(result.is_err());
}


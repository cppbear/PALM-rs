// Answer 0

#[test]
fn test_remove_entry() {
    struct TestEntries<K, V> {
        data: Vec<(K, V)>,
    }

    impl<K: Hash + Eq, V> Entries for TestEntries<K, V> {
        type Entry = (K, V);

        fn into_entries(self) -> Vec<Self::Entry> {
            self.data
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.data
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.data
        }

        fn with_entries<F>(&mut self, f: F) where F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries { data: vec![(1, "a"), (2, "b"), (3, "c")] };
    let index = hashbrown::hash_table::OccupiedEntry::from_index(&mut entries.data, 1);
    let mut entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index,
        hash_builder: PhantomData,
    };

    let (key, value) = entry.remove_entry();
    assert_eq!(key, 2);
    assert_eq!(value, "b");
    assert_eq!(entries.as_entries(), &[(1, "a"), (3, "c")]);
}

#[test]
fn test_remove_entry_empty() {
    struct TestEntries<K, V> {
        data: Vec<(K, V)>,
    }

    impl<K: Hash + Eq, V> Entries for TestEntries<K, V> {
        type Entry = (K, V);

        fn into_entries(self) -> Vec<Self::Entry> {
            self.data
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.data
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.data
        }

        fn with_entries<F>(&mut self, f: F) where F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries { data: vec![] };
    let index = hashbrown::hash_table::OccupiedEntry::from_index(&mut entries.data, 0);
    let mut entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index,
        hash_builder: PhantomData,
    };

    // Assuming an empty entry should panic when trying to remove.
    #[should_panic(expected = "index out of bounds")]
    let _ = entry.remove_entry();
}


// Answer 0

#[test]
fn test_swap_remove_entry_non_empty() {
    struct TestEntries {
        data: Vec<Bucket<i32, String>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, String>;

        fn into_entries(self) -> Vec<Self::Entry> {
            self.data
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.data
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.data
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries {
        data: vec![
            Bucket { hash: 1.into(), key: 1, value: "one".to_string() },
            Bucket { hash: 2.into(), key: 2, value: "two".to_string() },
            Bucket { hash: 3.into(), key: 3, value: "three".to_string() },
        ],
    };

    let index = hash_table::OccupiedEntry::from_index(&mut entries, 1);
    let entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index,
        hash_builder: PhantomData,
    };

    let (key, value) = entry.swap_remove_entry();
    assert_eq!(key, 2);
    assert_eq!(value, "two".to_string());
}

#[test]
#[should_panic]
fn test_swap_remove_entry_empty() {
    struct TestEntries {
        data: Vec<Bucket<i32, String>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, String>;

        fn into_entries(self) -> Vec<Self::Entry> {
            self.data
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.data
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.data
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.data);
        }
    }

    let mut entries = TestEntries { data: vec![] };
    let index = hash_table::OccupiedEntry::from_index(&mut entries, 0);

    let entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index,
        hash_builder: PhantomData,
    };

    // This should panic since there are no entries to swap-remove
    entry.swap_remove_entry();
}


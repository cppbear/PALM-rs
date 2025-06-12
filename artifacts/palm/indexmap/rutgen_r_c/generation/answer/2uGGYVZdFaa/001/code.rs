// Answer 0

#[test]
fn test_shift_remove_entry() {
    struct TestEntries {
        entries: Vec<Bucket<i32, String>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, String>;

        fn into_entries(self) -> Vec<Self::Entry> {
            self.entries
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.entries
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.entries
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.entries)
        }
    }

    pub struct TestRawOccupiedEntryMut<'a> {
        entries: &'a mut TestEntries,
        index: usize,
    }

    impl<'a> TestRawOccupiedEntryMut<'a> {
        fn new(entries: &'a mut TestEntries, index: usize) -> Self {
            Self { entries, index }
        }

        fn shift_remove_entry(self) -> (i32, String) {
            let index = self.index;
            let entry = self.entries.entries.remove(index);
            (entry.key, entry.value)
        }
    }

    let mut entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue::new(1), key: 1, value: "One".to_string() },
            Bucket { hash: HashValue::new(2), key: 2, value: "Two".to_string() },
            Bucket { hash: HashValue::new(3), key: 3, value: "Three".to_string() },
        ],
    };

    let occupied_entry = TestRawOccupiedEntryMut::new(&mut entries, 1);
    let (key, value) = occupied_entry.shift_remove_entry();

    assert_eq!(key, 2);
    assert_eq!(value, "Two");
    assert_eq!(entries.entries.len(), 2);
    assert_eq!(entries.entries[0].key, 1);
    assert_eq!(entries.entries[1].key, 3);
}

#[test]
#[should_panic]
fn test_shift_remove_entry_panic() {
    struct TestEntries {
        entries: Vec<Bucket<i32, String>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, String>;

        fn into_entries(self) -> Vec<Self::Entry> {
            self.entries
        }

        fn as_entries(&self) -> &[Self::Entry] {
            &self.entries
        }

        fn as_entries_mut(&mut self) -> &mut [Self::Entry] {
            &mut self.entries
        }

        fn with_entries<F>(&mut self, f: F)
        where
            F: FnOnce(&mut [Self::Entry]),
        {
            f(&mut self.entries)
        }
    }

    pub struct TestRawOccupiedEntryMut<'a> {
        entries: &'a mut TestEntries,
        index: usize,
    }

    impl<'a> TestRawOccupiedEntryMut<'a> {
        fn new(entries: &'a mut TestEntries, index: usize) -> Self {
            Self { entries, index }
        }

        fn shift_remove_entry(self) -> (i32, String) {
            let index = self.index;
            let entry = self.entries.entries.remove(index);
            (entry.key, entry.value)
        }
    }

    let mut entries = TestEntries { entries: vec![] };
    let occupied_entry = TestRawOccupiedEntryMut::new(&mut entries, 0);
    occupied_entry.shift_remove_entry();  // This should panic since the entries are empty.
}


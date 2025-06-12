// Answer 0

#[test]
fn test_get() {
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
            f(&mut self.entries);
        }
    }

    let mut entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue::new(1), key: 1, value: "a".to_string() },
            Bucket { hash: HashValue::new(2), key: 2, value: "b".to_string() },
        ],
    };

    let occupied_entry_index = hashbrown::hash_table::OccupiedEntry::from_index(0);
    let occupied_entry = OccupiedEntry::new(&mut entries, occupied_entry_index);

    assert_eq!(occupied_entry.get(), &"a".to_string());

    entries.entries[0].value = "changed".to_string();
    assert_eq!(occupied_entry.get(), &"changed".to_string());
}


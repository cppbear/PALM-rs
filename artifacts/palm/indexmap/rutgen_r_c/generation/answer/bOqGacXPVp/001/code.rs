// Answer 0

#[test]
fn test_occupied_entry_debug_fmt() {
    use crate::HashValue;
    use core::fmt::Write;

    struct MockEntry<K, V> {
        key: K,
        value: V,
    }

    struct MockEntries {
        entries: Vec<MockEntry<i32, String>>,
    }

    impl Entries for MockEntries {
        type Entry = MockEntry<i32, String>;

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

    let mut entries = MockEntries { entries: vec![
        MockEntry { key: 1, value: "Value 1".to_string() },
        MockEntry { key: 2, value: "Value 2".to_string() },
    ]};

    let occupied_entry_index = hashbrown::hash_table::OccupiedEntry::new(0);
    let occupied_entry = OccupiedEntry::new(&mut entries, occupied_entry_index);

    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{:?}", occupied_entry);
    
    assert_eq!(
        buffer,
        "OccupiedEntry { key: 1, value: \"Value 1\" }"
    );
}

#[test]
fn test_occupied_entry_debug_fmt_empty() {
    use crate::HashValue;
    use core::fmt::Write;

    struct MockEntry<K, V> {
        key: K,
        value: V,
    }

    struct MockEntries {
        entries: Vec<MockEntry<i32, String>>,
    }

    impl Entries for MockEntries {
        type Entry = MockEntry<i32, String>;

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

    let mut entries = MockEntries { entries: vec![] };

    let result = std::panic::catch_unwind(|| {
        let occupied_entry_index = hashbrown::hash_table::OccupiedEntry::new(0);
        let occupied_entry = OccupiedEntry::new(&mut entries, occupied_entry_index);
        let _ = write!(&mut String::new(), "{:?}", occupied_entry);
    });

    assert!(result.is_err());
}


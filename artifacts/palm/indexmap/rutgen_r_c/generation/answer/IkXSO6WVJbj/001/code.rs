// Answer 0

#[test]
fn test_key_valid_entry() {
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

    let mut test_entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue::default(), key: 1, value: String::from("one") },
            Bucket { hash: HashValue::default(), key: 2, value: String::from("two") },
        ],
    };

    let index = hashbrown::hash_table::OccupiedEntry::new(&mut test_entries.entries, 0);
    let occupied_entry = OccupiedEntry::new(&mut test_entries, index);
    
    assert_eq!(occupied_entry.key(), &1);
}

#[test]
#[should_panic]
fn test_key_out_of_bounds() {
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

    let mut test_entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue::default(), key: 1, value: String::from("one") },
        ],
    };

    let index = hashbrown::hash_table::OccupiedEntry::new(&mut test_entries.entries, 1); // Out of bounds index
    let occupied_entry = OccupiedEntry::new(&mut test_entries, index);
    
    occupied_entry.key(); // Should panic here
}


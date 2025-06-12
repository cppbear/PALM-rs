// Answer 0

#[test]
fn test_key_mut_valid_case() {
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
            Bucket { hash: HashValue::zero(), key: 1, value: "Value1".to_string() },
            Bucket { hash: HashValue::zero(), key: 2, value: "Value2".to_string() },
        ],
    };
    let index = 0;
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(&mut entries.data, index).unwrap());

    let key_mut = occupied_entry.key_mut();
    *key_mut = 3;
    assert_eq!(entries.data[index].key, 3);
}

#[should_panic]
fn test_key_mut_out_of_bounds() {
    struct PanicEntries {
        data: Vec<Bucket<i32, String>>,
    }

    impl Entries for PanicEntries {
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

    let mut entries = PanicEntries {
        data: Vec::new(),
    };
    let index = 0;
    let occupied_entry = OccupiedEntry::new(&mut entries, hash_table::OccupiedEntry::new(&mut entries.data, index).unwrap());

    occupied_entry.key_mut();
}


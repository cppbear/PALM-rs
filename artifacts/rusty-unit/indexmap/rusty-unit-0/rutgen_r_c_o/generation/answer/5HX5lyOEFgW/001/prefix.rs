// Answer 0

#[test]
fn test_into_ref_mut_valid_input() {
    struct TestEntries {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, i32>;

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
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.entries);
        }
    }

    let mut entries = TestEntries { entries: Vec::new() };
    entries.entries.push(Bucket { hash: HashValue::default(), key: 1, value: 10 }); // not empty

    let index = hash_table::OccupiedEntry::from_index(0); // valid index
    let occupied_entry = OccupiedEntry::new(&mut entries, index);
    let _ref_mut = occupied_entry.into_ref_mut();
}

#[test]
#[should_panic]
fn test_into_ref_mut_empty_entries() {
    struct TestEntries {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, i32>;

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
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.entries);
        }
    }

    let mut entries = TestEntries { entries: Vec::new() }; // empty entries

    let index = hash_table::OccupiedEntry::from_index(0); // valid index
    let occupied_entry = OccupiedEntry::new(&mut entries, index);
    let _ref_mut = occupied_entry.into_ref_mut(); // should panic
}

#[test]
fn test_into_ref_mut_valid_index() {
    struct TestEntries {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, i32>;

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
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.entries);
        }
    }

    let mut entries = TestEntries { entries: Vec::new() };
    entries.entries.push(Bucket { hash: HashValue::default(), key: 2, value: 20 }); // not empty

    let index = hash_table::OccupiedEntry::from_index(0); // valid index
    let occupied_entry = OccupiedEntry::new(&mut entries, index);
    let _ref_mut = occupied_entry.into_ref_mut();
}

#[test]
#[should_panic]
fn test_into_ref_mut_invalid_index() {
    struct TestEntries {
        entries: Vec<Bucket<i32, i32>>,
    }

    impl Entries for TestEntries {
        type Entry = Bucket<i32, i32>;

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
            F: FnOnce(&mut [Self::Entry]) {
            f(&mut self.entries);
        }
    }

    let mut entries = TestEntries { entries: Vec::new() };
    entries.entries.push(Bucket { hash: HashValue::default(), key: 3, value: 30 }); // not empty

    let index = hash_table::OccupiedEntry::from_index(1); // invalid index
    let occupied_entry = OccupiedEntry::new(&mut entries, index);
    let _ref_mut = occupied_entry.into_ref_mut(); // should panic
}


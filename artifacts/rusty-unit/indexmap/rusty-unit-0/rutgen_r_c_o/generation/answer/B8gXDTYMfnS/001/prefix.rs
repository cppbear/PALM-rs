// Answer 0

#[derive(Debug)]
struct TestEntries {
    entries: Vec<Bucket<String, i32>>,
}

impl Entries for TestEntries {
    type Entry = Bucket<String, i32>;

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

#[test]
fn test_key_mut_valid_index() {
    let mut entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue(1), key: String::from("key1"), value: 10 },
            Bucket { hash: HashValue(2), key: String::from("key2"), value: 20 },
        ]
    };
    let mut occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::Occupied(0),
        hash_builder: PhantomData,
    };
    let key_mut_ref = occupied_entry.key_mut();
}

#[test]
fn test_key_mut_last_index() {
    let mut entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue(3), key: String::from("key3"), value: 30 },
        ]
    };
    let mut occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::Occupied(0),
        hash_builder: PhantomData,
    };
    let key_mut_ref = occupied_entry.key_mut();
}

#[test]
fn test_key_mut_empty_entries() {
    let mut entries = TestEntries {
        entries: Vec::new(),
    };
    let mut occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::Occupied(0),
        hash_builder: PhantomData,
    };
    let key_mut_ref = occupied_entry.key_mut();
}

#[test]
#[should_panic]
fn test_key_mut_invalid_index() {
    let mut entries = TestEntries {
        entries: vec![
            Bucket { hash: HashValue(4), key: String::from("key4"), value: 40 },
        ]
    };
    let mut occupied_entry = RawOccupiedEntryMut {
        entries: &mut entries,
        index: hash_table::OccupiedEntry::Occupied(1), // Invalid index
        hash_builder: PhantomData,
    };
    let key_mut_ref = occupied_entry.key_mut();
}

